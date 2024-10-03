use crate::{admin::FormCred, auth::hash_password, forms::FormField};
use bb8_redis::{bb8::Pool, redis, RedisConnectionManager};
use chrono::{DateTime, NaiveDateTime, Utc};
use dotenvy::dotenv;
use redis::AsyncCommands;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;

//userid,email,username,passkey
#[derive(Debug, Deserialize, FromRedisValue, Serialize, ToRedisArgs)]
pub struct User {
    pub userid: i32,
    pub email: String,
    pub username: String,
    pub passkey: String,
}

#[derive(Debug, Deserialize, FromRedisValue, Serialize, ToRedisArgs)]
pub struct FormData {
    pub fid: i32,
    pub gid: String,
    pub fields: Vec<FormField>,
}

pub async fn get_db_conn_pool() -> sqlx::Pool<Postgres> {
    dotenv().ok();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("env variable DATABASE_URL must be set!"))
        .await
        .expect("Unable to create a connection pool")
}

pub async fn setup_db(conn: &sqlx::Pool<Postgres>) {
    sqlx::migrate!("./migrations")
        .run(conn)
        .await
        .expect("Unable to perform migrations");

    let mut transaction = conn.begin().await.expect("Unable to get transaction lock");

    let admin_email =
        &env::var("DEFAULT_ADMIN_MAIL").expect("env variable DEFAULT_ADMIN_MAIL must be set!");

    let admin_passkey =
        &env::var("DEFAULT_ADMIN_KEY").expect("env variable DEFAULT_ADMIN_KEY must be set!");

    let admin_hash = hash_password(
        &env::var("DEFAULT_ADMIN_PASSWORD")
            .expect("env variable DEFAULT_ADMIN_PASSWORD must be set!"),
    )
    .await;

    sqlx::query("INSERT INTO admins(aid,email,username,passhash) VALUES(0,$1,'ADMIN',$2) ON CONFLICT DO NOTHING;")
        .bind(admin_email)
        .bind(admin_hash)
        .execute(&mut *transaction)
        .await
        .expect("Unable to create DEFAULT ADMIN in admins table");

    sqlx::query("INSERT INTO forms_user(userid,email,username,passkey) VALUES(0,$1,'ADMIN',$2) ON CONFLICT DO NOTHING;")
        .bind(admin_email)
        .bind(admin_passkey)
        .execute(&mut *transaction)
        .await
        .expect("Unable to create DEFAULT ADMIN in forms_user table");

    sqlx::query("INSERT INTO user_group(uqid,userid,gid) VALUES(0,0,1) ON CONFLICT DO NOTHING;")
        .execute(&mut *transaction)
        .await
        .expect("Unable to create DEFAULT user_group in forms_user table");

    sqlx::query("INSERT INTO form_register(fid,gid,form_name) VALUES(0,0,'Test_Form') ON CONFLICT DO NOTHING;")
        .execute(&mut *transaction)
        .await
        .expect("Unable to create DEFAULT form in forms table");

    sqlx::query("INSERT INTO forms(elid,fid,typ,req,field_name,question) VALUES(0,0,'text',true,'name','What is your name?'),(1,'0000','email',true,'email','What is your Email?') ON CONFLICT DO NOTHING;")
        .execute(&mut *transaction)
        .await
        .expect("Unable to create DEFAULT form entries in forms table");

    transaction
        .commit()
        .await
        .expect("Unable to complete setup transactions");
}

pub async fn ping_db(conn: &sqlx::Pool<Postgres>) -> bool {
    let row: (i64,) = sqlx::query_as("SELECT $1;")
        .bind(150_i64)
        .fetch_one(conn)
        .await
        .expect("Unable to make test query");
    row.0 == 150
}

pub async fn retrieve_admin(
    conn: sqlx::Pool<Postgres>,
    e_mail: String,
) -> Result<(i32, String, String), sqlx::Error> {
    sqlx::query_as("SELECT aid,username,passhash FROM admins WHERE email=$1 ;")
        .bind(e_mail)
        .fetch_one(&conn)
        .await
}

pub async fn retrieve_user(
    conn: &sqlx::Pool<Postgres>,
    key: String,
) -> Result<(i32, String, String), sqlx::Error> {
    sqlx::query_as("SELECT userid,username,email FROM forms_user WHERE passkey=$1 ;")
        .bind(key)
        .fetch_one(conn)
        .await
}

pub async fn redis_load(conn: &sqlx::Pool<Postgres>, redis_pool: &Pool<RedisConnectionManager>) {
    let mut redis_conn = redis_pool
        .get()
        .await
        .expect("Unable to acquire connection for redis");

    // Fetches user data from db and writes to redis
    print!("  Setting up Redis User cache  : ");
    let mut ucount = 0;
    let user_data_from_db =
        sqlx::query_as::<_, (i32, String, String, String)>("SELECT * FROM forms_user")
            .fetch_all(conn)
            .await
            .expect("Unable to fetch for copy");

    for (userid, email, username, passkey) in user_data_from_db {
        ucount += 1;

        let user = User {
            userid,
            email,
            username,
            passkey: passkey.clone(),
        };

        let passkey = format!("{passkey}_Userkey");
        redis_conn
            .set::<&str, &User, ()>(&passkey, &user)
            .await
            .expect("Unable to load db into memory");
    }
    println!("Loaded {ucount} user(s) data into Memory.");

    // Fetches forms data from db and writes to redis
    print!("  Setting up Redis Forms cache : ");
    let mut fcount = 0;
    let form_register_vals_from_db =
        sqlx::query_as::<_, (i32, i32)>("SELECT fid,gid FROM form_register")
            .fetch_all(conn)
            .await
            .expect("Unable to fetch from Database");

    for (form_id, group_id) in form_register_vals_from_db {
        let mut form_fields: Vec<FormField> = vec![];
        fcount += 1;

        let form_fields_from_db =
            sqlx::query_as("SELECT fid,typ,field_name,question FROM forms WHERE fid = $1;")
                .bind::<&i32>(&form_id)
                .fetch_all(conn)
                .await
                .expect("Unable to fetch from Database");

        for (fid, typ, fname, question) in form_fields_from_db {
            let forms = FormField {
                fid,
                typ,
                fname,
                question,
            };

            form_fields.push(forms);
        }

        // Creating the formdata value
        let formkey = format!("{}_Formkey", &form_id);
        let formdat: FormData = FormData {
            fid: form_id,
            gid: group_id.to_string(),
            fields: form_fields,
        };

        // Insert the key(fid) and Value(formdata) pair into redis
        redis_conn
            .set::<&str, &FormData, ()>(&formkey, &formdat)
            .await
            .expect("Unable to load db into memory");
    }

    println!("Loaded {fcount} form(s) data into Memory.")
}

pub async fn get_form_fields(conn: &sqlx::Pool<Postgres>, form_id: &String) -> Vec<FormField> {
    let all_fields: Vec<(i32, String, String, String)> =
        sqlx::query_as("SELECT fid,typ,field_name,question FROM forms WHERE fid = $1;")
            .bind(form_id)
            .fetch_all(conn)
            .await
            .expect("Unable to fetch from Database");
    let mut vec: Vec<FormField> = vec![];
    for i in all_fields {
        vec.push(FormField {
            fid: i.0,
            typ: i.1,
            fname: i.2,
            question: i.3,
        });
    }
    vec
}

pub async fn new_form_with_id(conn: &sqlx::Pool<Postgres>,data: FormCred)-> i32{
    let mut _transaction = conn.begin().await.expect("Unable to get transaction lock");

    let dstart = NaiveDateTime::parse_from_str(&data.start, "%Y-%m-%dT%H:%M").expect("Invalid date format");
    let dend = NaiveDateTime::parse_from_str(&data.end, "%Y-%m-%dT%H:%M").expect("Invalid date format");
    
    let _datetime_s: chrono::DateTime<chrono::Utc> = dstart.and_utc();
    let _datetime_e: chrono::DateTime<chrono::Utc> = dend.and_utc();

    
    /*sqlx::query("INSERT INTO form_register(gid, form_name, form_description, startdate, starttime, enddate, endtime) VALUES($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING;")
        .bind(data.gid)
        .bind(data.name)
        .bind(data.desc)
        .bind(datetime_s.date_naive())
        .bind(datetime_s.time())
        .bind(datetime_e.date_naive())
        .bind(datetime_e.time())
        .execute(&mut *transaction)
        .await
        .expect("Unable to create New Form Entry in table");
    
    transaction
        .commit()
        .await
        .expect("Unable to complete setup transactions");

    let id = sqlx::query_as::<Postgres, (i32,)>("SELECT fid FROM form_register TAIL 1")
        .fetch_one(conn)
        .await
        .expect("Unable to fetch for copy");
    */
    10

}
