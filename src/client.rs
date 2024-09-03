use askama_axum::{IntoResponse, Template};
use axum::{body::Body, http::Response, response::Redirect, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeFile;
use crate::jwt_auth::verify_cookie; // bring trait in scope
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
    name: &'a str,
}

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormsTemplate<'a> {
    id: &'a str,
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


pub fn service_router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/login", get(login))
        .route("/forms", get(forms))
        .route("/admin", get(admin))
        .route_service(
            "/output.css",
            ServeFile::new("./templates/assets/output.css"),
        )
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn home(cookies: Cookies) -> Response<Body> {
    if !verify_cookie(&cookies).await.0 {
        return to_login().await;
    }
    let home = HelloTemplate { name: "world" }; // instantiate your struct
    home.into_response()
}

pub async fn login() -> Response<Body> {
    let login = LoginTemplate { name: "world" }; // instantiate your struct
    login.into_response()
}

pub async fn forms(cookies: Cookies) -> Response<Body> {
    if !verify_cookie(&cookies).await.0 {
        return to_login().await;
    }
    let forms = FormsTemplate { id: "12e4" }; // instantiate your struct
    forms.into_response()
}

pub async fn admin(cookies: Cookies) -> Response<Body> {
    if !verify_cookie(&cookies).await.1{
        return to_home().await;
    }
    let forms = AdminTemplate { name: "Hello" }; // instantiate your struct
    forms.into_response()
}


pub async fn to_login() -> Response<Body> {
    Redirect::to("/login").into_response()
}

pub async fn to_home() -> Response<Body> {
    Redirect::to("/").into_response()
}