use dotenv::dotenv;
use sea_orm::Database;
use std::env;

pub async fn ping_db(){
    dotenv().ok();
    let _db = Database::connect(
        env::var("DATABASE_URL").expect("env variable DATABASE_URL must be set!"),
    )
    .await;
    println!("Connected to Postgres");
}
