use axum::Router;
use std::net::SocketAddr;

pub mod router;
use router::using_serve_dir;

//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
//use tower_http::trace::TraceLayer;

const ADDR: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 8000;

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
    println!("Starting Axum Super forms Server.");
    tokio::join!(serve(using_serve_dir(), PORT),);
}



async fn serve(app: Router, port: u16) {
    println!("Serving on address: http://127.0.0.1:{PORT}");
    let addr = SocketAddr::from((ADDR, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app
        
    )
        .await
        .unwrap();
}
