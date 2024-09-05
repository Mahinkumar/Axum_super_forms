use askama_axum::{IntoResponse, Template};
use axum::{body::Body, http::Response, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};
use crate::{auth::admin_login_handler, jwt_auth::verify_cookie, router::{to_home, to_login}};


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



pub fn admin_router() -> Router {
    Router::new()
        .route("/admin", get(admin))
        .route("/admin/login", get(admin_login))
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn admin(cookies: Cookies) -> Response<Body> {
    if verify_cookie(&cookies).await.1{
        return to_home().await;
    }
    let forms = AdminTemplate { name: "Hello" }; // instantiate your struct
    forms.into_response()
}


pub async fn admin_login(cookies: Cookies) -> Response<Body> {
    if verify_cookie(&cookies).await.0 {
        return to_login().await;
    }
    let admin_login = AdminLoginTemplate { message: "Enter your credentials" }; // instantiate your struct
    admin_login.into_response()
}