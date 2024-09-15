use crate::{
    forms::FormField, jwt_auth::{JWToken, Utype}, DbPools
};

struct FormFilled{
    value: String,
    fields: FormField,
}

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
use axum_extra::extract::cookie::CookieJar;
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

#[derive(Template)]
#[template(path = "adminnewform.html")]
pub struct AdminnewformTemplate<'a> {
    id: &'a str,
    el: Vec<FormFilled>,
}


pub fn admin_router() -> Router<DbPools> {
    Router::new()
        .route("/admin", get(admin))
        .route("/admin/form/new", get(admin_new_form))
        .layer(middleware::from_fn(admin_auth_middleware))
        .layer(CookieManagerLayer::new())
}

pub async fn admin() -> Response<Body> {
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

pub async fn admin_auth_middleware(request: Request, next: Next) -> Response<Body> {
    let jar = CookieJar::from_headers(request.headers());
    let cookie_unpacked = jar.get("Access_token_admin");

    let cookie = match cookie_unpacked {
        Some(_) => cookie_unpacked.expect("Failed to unwrap cookie jar"),
        None => return Redirect::to("/admin/login").into_response(),
    };

    if JWToken::validate_token(cookie.value().to_string())
        .await
        .is_admin
    {
        let response = next.run(request).await;
        return response;
    } else {
        return Redirect::to("/admin/login").into_response();
    }
}

pub async fn admin_new_form()-> Response<Body>{
    let els: Vec<FormFilled> = vec![];
    let formnew = AdminnewformTemplate{ el: els, id: "None"};
    formnew.into_response()
}
