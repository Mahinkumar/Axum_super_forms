use crate::{
    jwt_auth::{JWToken, Utype},
    DbPools,
};
use askama_axum::{IntoResponse, Template};
use axum::{body::Body, http::Response, response::Redirect, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};

#[derive(Template)]
#[template(path = "adminLogin.html")]
pub struct AdminLoginTemplate<'a> {
    message: &'a str,
}

#[derive(Template)]
#[template(path = "admin.html")]
pub struct AdminTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "adminForm.html")]
pub struct AdminFormTemplate<'a> {
    id: &'a str,
}

#[derive(Template)]
#[template(path = "adminstat.html")]
pub struct AdminStatTemplate<'a> {
    id: &'a str,
}

pub fn admin_router() -> Router<DbPools> {
    Router::new()
        .route("/admin", get(admin))
        .route("/admin/login", get(admin_login))
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn admin(cookies: Cookies) -> Response<Body> {
    let cookie_ver = JWToken::verify_cookie(&cookies, Utype::Admin).await;
    if !cookie_ver.is_admin {
        return Redirect::to("/admin/login").into_response();
    }
    let forms = AdminTemplate { name: "Admin" }; // instantiate your struct
    forms.into_response()
}

pub async fn admin_login(cookies: Cookies, mut message: String) -> Response<Body> {
    if message.is_empty() {
        message = "Enter your credentials".to_string();
    }
    if JWToken::verify_cookie(&cookies, Utype::Admin)
        .await
        .is_admin
    {
        return Redirect::to("/admin").into_response();
    }
    let admin_login = AdminLoginTemplate { message: &message }; // instantiate your struct
    admin_login.into_response()
}
