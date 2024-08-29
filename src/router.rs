use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{middleware, Router};
use axum::Form;
use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};
use rust_embed::Embed;
use serde::Deserialize;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use crate::auth::hash_password;

#[derive(Deserialize)]
struct Login{
    email: String,
    password: String,
}

use crate::jwt_auth::authorization_middleware;


static INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "./client/dist/"]
struct Assets;

//use tower_http::trace::TraceLayer;

pub fn service_router() -> Router {
    Router::new()
        .fallback(get(static_handler))
        .route("/login", post(login_handler).get(static_handler))
        .layer(middleware::from_fn(authorization_middleware))
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

async fn login_handler(uri: Uri,Form(login): Form<Login>) -> impl IntoResponse{
    let hash = hash_password(login.password.as_bytes());
    println!("Form from {} Posted {} and Password hash was generated",uri, login.email);
    println!("Hash : {}",hash);
    Redirect::to("/")
}

async fn static_handler(uri: Uri, cookie: Cookies) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            cookie.add(Cookie::new("Cookie_aka", "Cookie"));
            if path.contains('.') {
                return not_found().await;
            }
            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
