use axum::Router;
use std::net::SocketAddr;

pub mod router;
pub mod jwt_auth;
pub mod auth;
pub mod mem_kv;
pub mod client;
pub mod db;
pub mod admin;

use router::general_router;
use router::login_router;
use admin::admin_router;
use client::client_router;
use mem_kv::ping;
use db::ping_db;

//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
//use tower_http::trace::TraceLayer;

const ADDR: [u8; 4] = [127, 0, 0, 1];
const PORT_HOST: u16 = 8000;

#[tokio::main]


async fn main() {
    
    // Use for Debug only!! Heavily reduces perfomance
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    //             format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
    //         }),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    println!("=================================================================");
    println!("Starting Axum Super forms Server.");
    println!("Redis Server Status       : {}", if ping().await {"Active"} else {"Unable to connect"});
    println!("Postgres Server Status    : {}", if ping_db().await {"Active"} else {"Unable to connect"});
    
    let axum_router = Router::new()
        .merge(login_router())
        .merge(admin_router())
        .merge(general_router())
        .merge(client_router());

    tokio::join!(serve(axum_router, PORT_HOST));
}

async fn serve(app: Router, port: u16) {
    println!("Serving on address        : http://127.0.0.1:{port}");
    println!("=================================================================");
    let addr = SocketAddr::from((ADDR, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
