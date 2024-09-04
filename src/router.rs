use axum::routing::post;
use axum::Router;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use axum::response::Redirect;
use axum::Form;
use axum::{
    http::Uri,
    response::IntoResponse,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String,
}


//use tower_http::trace::TraceLayer;

pub fn api_router() -> Router {
    Router::new().route("/login", post(login_handler))
    .layer(CookieManagerLayer::new())
    .layer(CorsLayer::permissive())
}

async fn login_handler(_cookie: Cookies, uri: Uri, Form(login): Form<Login>) -> impl IntoResponse {
    println!(
        "Form from {} Posted {} and Password hash was generated",
        uri, login.email
    );
}

pub async fn embed_token(token: String, cookie: Cookies) {
    let mut auth_cookie = Cookie::new("access_token", token);
    auth_cookie.set_http_only(true);
    auth_cookie.set_secure(true);
    cookie.add(auth_cookie)
}
