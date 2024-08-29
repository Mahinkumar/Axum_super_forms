use axum::Router;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
//use bb8::{Pool, PooledConnection};
use bb8_redis::bb8;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

pub mod router;
pub mod auth;

use auth::{create_token,validate_token};
use router::service_router;

//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
//use tower_http::trace::TraceLayer;

const ADDR: [u8; 4] = [127, 0, 0, 1];
const PORT_HOST: u16 = 8000;

#[tokio::main]


async fn main() {
    dotenv().ok();
    // Use for Debug only!! Heavily reduces perfomance
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    //             format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
    //         }),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    println!("Starting Axum Super forms Server.");
    println!("Connecting to Redis Backend ..");

    let manager = RedisConnectionManager::new(
        env::var("REDIS_CONNECTION_URL").expect("env variable REDIS_CONNECTION_URL must be set!"),
    )
    .unwrap();

    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    print!("Pinging Redis: ");

    {
        // ping the database before starting
        let mut conn = pool.get().await.unwrap();
        conn.set::<&str, &str, ()>("Check", "Response recieved!")
            .await
            .unwrap();
        let result: String = conn.get("Check").await.unwrap();
        println!("{}", result);
    }

    let token_test = create_token("Tester@mail.com", "Tester");
    validate_token(token_test);
    
    tokio::join!(serve(service_router(), PORT_HOST));
}

async fn serve(app: Router, port: u16) {
    println!("Serving on address: http://127.0.0.1:{port}");
    let addr = SocketAddr::from((ADDR, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
