use askama_axum::IntoResponse;
use axum::body::Body;
use axum::http::Response;
use axum::Router;
use axum::{response::Redirect, routing::post};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeFile;

use crate::auth::Login;
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
        .route("/login", post(Login::user_handler))
        .route("/admin/login", post(Login::admin_handler))
        .layer(CookieManagerLayer::new())
}


