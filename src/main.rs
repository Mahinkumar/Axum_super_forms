
use axum::{response::Html, routing::get,Router};
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tower_cookies::{Cookie,Cookies, CookieManagerLayer};


#[tokio::main]
async fn main() {
    tokio::join!(
        serve(using_serve_dir(), 3000),
    );
}

fn using_serve_dir() -> Router {
    // and thus it implements `Service`
    // serve the file in the "assets" directory under `/assets`
    Router::new()
        .route("/", get(home_handler))
        .route("/login", get(login_handler))
        .route_service("/assets", ServeDir::new("./frontend/dist/assets"))
        
        .layer(CookieManagerLayer::new())
    }

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn home_handler(cookies: Cookies) -> Html<String> {
    let html_content = 
    read_html_from_file("./frontend/dist/index.html").await.unwrap_or_else(|_| {
        "<h1>Error loading HTML file</h1>".to_string()
    });
    cookies.add(Cookie::new("key", "aka_123"));
    Html(html_content)
}

async fn login_handler(cookies: Cookies) -> Html<String> {
    let html_content = 
    read_html_from_file("./frontend/dist/login/index.html").await.unwrap_or_else(|_| {
        "<h1>Error loading HTML file</h1>".to_string()
    });
    cookies.add(Cookie::new("key", "aka_123"));
    Html(html_content)
}

async fn read_html_from_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}