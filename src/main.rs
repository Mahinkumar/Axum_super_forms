use axum::Router;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

const ADDR: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;
const FRONTEND_PATH: &str = "./client/dist";

#[tokio::main]
async fn main() {
    println!("Starting Axum Super forms Server.");
    tokio::join!(serve(using_serve_dir(), PORT),);
}

fn using_serve_dir() -> Router {
    // and thus it implements `Service`
    // serve the file in the "assets" directory under `/assets`
    Router::new()
        .nest_service(
            "/",
            ServeDir::new(format!("{FRONTEND_PATH}/"))
                .not_found_service(ServeFile::new(format!("{FRONTEND_PATH}/index.html"))),
        )
        .nest_service("/assets",ServeDir::new(format!("{FRONTEND_PATH}/assets")))
        .layer(CookieManagerLayer::new())
}

async fn serve(app: Router, port: u16) {
    println!("Serving on address: http://127.0.0.1:{PORT}");
    let addr = SocketAddr::from((ADDR, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
