use tower_http::cors::CorsLayer;
use axum::response::Redirect;
use axum::routing::get;
use axum::Json;
use axum::Router;
use axum::Form;
use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};
use rust_embed::Embed;
use serde::Deserialize;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};


#[derive(Deserialize)]
struct Login{
    email: String,
    password: String,
}

use crate::auth::hash_password;
use crate::jwt_auth::create_token;


static INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "./client/dist/"]
struct Assets;

//use tower_http::trace::TraceLayer;

pub fn service_router() -> Router {
    Router::new()
        .fallback(get(static_handler))
        .route("/login", get(static_handler).post(login_handler))
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::permissive())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub fn api_router() -> Router {
    Router::new()
        .route("/api", get(|| async {Json("NOT IMPLEMENTED")}))
}

async fn login_handler(cookie: Cookies,uri: Uri,Form(login): Form<Login>) -> impl IntoResponse{
    println!("Form from {} Posted {} and Password hash was generated",uri, login.email);
    let _hash = hash_password(login.password.as_bytes());
    let token = create_token(&login.email, &login.email);
    embed_token(token.await,cookie).await;
    Redirect::to("/")
}


pub async fn embed_token(token : String,cookie: Cookies){
    let mut auth_cookie = Cookie::new("access_token",token);
    auth_cookie.set_http_only(true);
    auth_cookie.set_secure(true);
    cookie.add(auth_cookie)
}

async fn static_handler(uri: Uri, cookie: Cookies) -> impl IntoResponse {
    let is_cookie = cookie.get("access_token").is_none();
    let path = uri.path().trim_start_matches('/');
    if path.is_empty() || path == INDEX_HTML {
        if is_cookie {return to_login().await}
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }
            return index_html().await;
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

async fn to_login() -> Response{
    Redirect::to("/login").into_response()
}