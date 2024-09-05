use askama_axum::{IntoResponse, Template};
use axum::{body::Body, http::Response, response::Redirect, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};
use crate::{jwt_auth::verify_cookie, router::to_login}; // bring trait in scope
                                     //use tower_http::cors::CorsLayer;

#[derive(Template)] // this will generate the code...
#[template(path = "home.html")]
pub struct HelloTemplate<'a> {
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
#[template(path = "form.html")]
pub struct FormsTemplate<'a> {
    id: &'a str,
}



pub fn client_router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/login", get(login))
        .route("/forms", get(forms))
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn home(cookies: Cookies) -> Response<Body> {
    let cookie_ver = verify_cookie(&cookies,"Access_token_user".to_string()).await;
    if !cookie_ver.0 {
        return to_login().await;
    }
    let home = HelloTemplate { name: "world" }; // instantiate your struct
    home.into_response()
}


pub async fn login(cookies: Cookies) -> Response<Body> {
    let cookie_ver = verify_cookie(&cookies,"Access_token_user".to_string()).await;
    if cookie_ver.0 {
        return Redirect::to("/").into_response();
    }
    let login = LoginTemplate { message: "Enter you 8-character Secret key" }; // instantiate your struct
    login.into_response()
}

pub async fn forms(cookies: Cookies) -> Response<Body> {
    let cookie_ver = verify_cookie(&cookies,"Access_token_user".to_string()).await;
    if !cookie_ver.0 {
        return to_login().await;
    }
    let forms = FormsTemplate { id: "12e4" }; // instantiate your struct
    forms.into_response()
}



