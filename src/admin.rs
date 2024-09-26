use crate::{
    forms::{FormField, FormInput}, jwt_auth::{JWToken, Utype}, DbPools
};

struct FormFilled{
    value: String,
    fields: FormField,
}

use askama_axum::{IntoResponse, Template};
use axum::{
    body::Body,
    extract::{Request, State},
    http::Response,
    middleware::{self, Next},
    response::Redirect,
    routing::get,
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use tower_cookies::{CookieManagerLayer, Cookies};

#[derive(Template)]
#[template(path = "admin/adminLogin.html")]
pub struct AdminLoginTemplate<'a> {
    message: &'a str,
}

#[derive(Template)]
#[template(path = "admin/admin.html")]
pub struct AdminTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "admin/site_config.html")]
pub struct ConfigTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "admin/adminForm.html")]
pub struct AdminFormTemplate<'a> {
    id: &'a str,
}

#[derive(Template)]
#[template(path = "admin/adminstat.html")]
pub struct AdminStatTemplate<'a> {
    id: &'a str,
}

#[derive(Template)]
#[template(path = "admin/adminnewform.html")]
pub struct AdminnewformTemplate{
    el: Vec<FormFilled>,
}

#[derive(Template)]
#[template(path = "admin/form_edit.html")]
pub struct AdmineditformTemplate{
    el: Vec<FormFilled>,
}


pub fn admin_router() -> Router<DbPools> {
    Router::new()
        .route("/admin/form/new", get(admin_new_form).post(admin_new_form_post))
        .route("/admin/form/edit/:id", get(edit_form))
        .route("/admin", get(admin))
        .route("/admin/siteconfig", get(siteconfig))
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
        response
    } else {
        Redirect::to("/admin/login").into_response()
    }
}

pub async fn admin_new_form()-> Response<Body>{
    let els: Vec<FormFilled> = vec![];
    let formnew = AdminnewformTemplate{ el: els };
    formnew.into_response()
}

pub async fn siteconfig()->Response<Body>{
    let config = ConfigTemplate{ name: "Admin"};
    config.into_response()
}

pub async fn admin_new_form_post(
    State(_db_pools): State<DbPools>,
    body: String) -> Response<Body>{
    let v: Vec<&str> = body.rsplit('&').collect();
    let mut form_inputs: Vec<FormInput> = vec![];
    for i in v {
        let kv = i.rsplit_once("=").expect("Unable to split");
        if kv.0 == "action"{
            if kv.1 == "Add"{
                println!("Form New | Add command Recieved");
            }else if kv.1 == "Finish"{
                println!("Form New | Finish command Recieved");
            }
        }
        let item: FormInput = FormInput {
            name: kv.0.to_string(),
            value: kv.1.to_string(),
        };
        form_inputs.push(item);
    }
    // We will redraw the forms for every add. 
    // Redirect to admin is only for finish command.
    Redirect::to("/admin").into_response()
}

pub async fn edit_form()-> Response<Body>{
   let page =  AdmineditformTemplate {el: vec![]};
   page.into_response()
}