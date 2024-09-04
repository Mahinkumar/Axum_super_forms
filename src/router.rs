use askama_axum::IntoResponse;
use axum::body::Body;
use axum::http::Response;
use axum::{response::Redirect, routing::post};
use axum::Router;
use tower_cookies::{Cookie, Cookies};
use tower_http::services::ServeFile;

use crate::auth::{admin_login_handler, login_handler};


//use tower_http::trace::TraceLayer;

pub fn general_router() -> Router {
    Router::new()
        .route_service(
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

pub fn login_router() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/admin/login", post(admin_login_handler))
}

pub async fn embed_token(token: String, cookie: Cookies) {
    let mut auth_cookie = Cookie::new("access_token", token);
    auth_cookie.set_http_only(true);
    auth_cookie.set_secure(true);
    cookie.add(auth_cookie)
}

