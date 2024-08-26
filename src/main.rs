use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::{services::ServeDir, trace::TraceLayer};

const ADDR: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    println!("Starting Axum Super forms Server.");
    tokio::join!(serve(using_serve_dir(), PORT),);
}

fn using_serve_dir() -> Router {
    // and thus it implements `Service`
    // serve the file in the "assets" directory under `/assets`
    Router::new()
        .route("/", get(home_handler))
        .route("/login", get(login_handler))
        .nest_service("/assets", ServeDir::new("./frontend/dist/assets"))
        .fallback(get(|| async { "404 Page Not found" }))
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

async fn home_handler(cookies: Cookies) -> Html<String> {
    let html_content = read_html_from_file("./frontend/dist/index.html")
        .await
        .unwrap_or_else(|_| "<h1>Error loading HTML file</h1>".to_string());
    cookies.add(Cookie::new("key", "aka_123"));
    Html(html_content)
}

async fn login_handler(cookies: Cookies) -> Html<String> {
    let html_content = read_html_from_file("./frontend/dist/login/index.html")
        .await
        .unwrap_or_else(|_| "<h1>Error loading HTML file</h1>".to_string());
    let mut cookie = Cookie::new("Session Token", "XFASFACAFASFASFASFAFA");
    cookie.set_same_site(SameSite::Strict);
    cookies.add(cookie);
    Html(html_content)
}

async fn read_html_from_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
