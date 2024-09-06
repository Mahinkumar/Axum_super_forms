use askama_axum::IntoResponse;
use axum::body::Body;
use axum::http::Response;
use axum::Router;
use axum::{response::Redirect, routing::post};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::services::ServeFile;

use crate::auth::{admin_login_handler, login_handler};
use crate::DbPools;

//use tower_http::trace::TraceLayer;

pub fn general_router() -> Router<DbPools> {
    Router::new().route_service(
        "/output.css",
        ServeFile::new("./templates/assets/output.css"),
    )
}

pub async fn to_login() -> Response<Body> {
    Redirect::to("/login").into_response()
}

pub async fn to_home() -> Response<Body> {
    Redirect::to("/").into_response()
}

pub fn login_router() -> Router<DbPools> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/admin/login", post(admin_login_handler))
        .layer(CookieManagerLayer::new())
}

pub async fn embed_token(token_name: String, token: String, cookie: Cookies) {
    let mut auth_cookie = Cookie::new(token_name, token);
    auth_cookie.set_http_only(true);
    auth_cookie.set_secure(true);
    cookie.add(auth_cookie)
}
