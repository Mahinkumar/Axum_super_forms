use crate::{jwt_auth::verify_cookie, router::to_login, DbPools};
use askama_axum::{IntoResponse, Template};
use axum::{body::Body, http::Response, response::Redirect, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};

#[derive(Template)] // this will generate the code...
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    message: &'a str,
}


#[derive(Template)]
#[template(path = "404.html")]
pub struct Page404Template<'a> {
    message: &'a str,
}

pub fn client_router() -> Router<DbPools> {
    Router::new()
        .route("/", get(home))
        .route("/login", get(login))
        .merge(route404())
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub fn route404() -> Router<DbPools> {
    Router::new().fallback_service(get(handle_404))
}

pub async fn handle_404() -> Response<Body> {
    let page404 = Page404Template { message: "" }; // instantiate your struct
    page404.into_response()
}

pub async fn home(cookies: Cookies) -> Response<Body> {
    let cookie_ver = verify_cookie(&cookies, "Access_token_user".to_string()).await;
    if !cookie_ver.0 {
        return to_login().await;
    }
    let home = HomeTemplate { name: "User" }; // instantiate your struct
    home.into_response()
}

pub async fn login(cookies: Cookies, mut message: String) -> Response<Body> {
    if message.is_empty() {
        message = "Enter your 8 digit key".to_string();
    }
    let cookie_ver = verify_cookie(&cookies, "Access_token_user".to_string()).await;
    if cookie_ver.0 {
        return Redirect::to("/").into_response();
    }
    let login = LoginTemplate { message: &message }; // instantiate your struct
    login.into_response()
}

