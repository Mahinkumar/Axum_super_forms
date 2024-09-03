
use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenv::dotenv;
use std::env;

pub async fn ping_db(){
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url).await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    println!("Postgres Active: true");
}