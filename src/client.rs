use crate::{
    jwt_auth::{JWToken, Utype},
    DbPools,
};
use askama_axum::{IntoResponse, Template};
use axum::{
    body::Body,
    extract::Request,
    http::Response,
    middleware::{self, Next},
    response::Redirect,
    routing::get,
    Router,
};
use axum_extra::extract::CookieJar;
use tower_cookies::{CookieManagerLayer, Cookies};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    name: &'a str,
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
        .merge(route404())
        .layer(middleware::from_fn(client_auth_middleware))
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

pub async fn home() -> Response<Body> {
    let home = HomeTemplate { name: "User" }; // instantiate your struct
    home.into_response()
}

pub async fn login(cookies: Cookies, mut message: String) -> Response<Body> {
    if message.is_empty() {
        message = "Enter your 8 digit key".to_string();
    }
    let cookie_ver = JWToken::verify_cookie(&cookies, Utype::User).await;
    if cookie_ver.is_user {
        return Redirect::to("/").into_response();
    }
    let login = LoginTemplate { message: &message }; // instantiate your struct
    login.into_response()
}

pub async fn client_auth_middleware(request: Request, next: Next) -> Response<Body> {
    let jar = CookieJar::from_headers(request.headers());
    let cookie_unpacked = jar.get("Access_token_user");

    let cookie = match cookie_unpacked {
        Some(_) => cookie_unpacked.expect("Failed to unwrap cookie jar"),
        None => return Redirect::to("/login").into_response(),
    };

    if JWToken::validate_token(cookie.value().to_string())
        .await
        .is_user
    {
        let response = next.run(request).await;
        return response;
    } else {
        return Redirect::to("/login").into_response();
    }
}
