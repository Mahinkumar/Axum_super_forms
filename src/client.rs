use askama_axum::Template;
use axum::{routing::get, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, services::ServeFile}; // bring trait in scope

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
    name: &'a str,
}

pub fn service_router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/login", get(login))
        .route("/forms", get(forms))
        .route_service(
            "/output.css",
            ServeFile::new("./templates/assets/output.css"),
        )
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::permissive())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn home() -> HelloTemplate<'static> {
    let home = HelloTemplate { name: "world" }; // instantiate your struct
    home
}

pub async fn login() -> LoginTemplate<'static> {
    let login = LoginTemplate { name: "world" }; // instantiate your struct
    login
}

pub async fn forms() -> FormsTemplate<'static> {
    let forms = FormsTemplate { name: "world" }; // instantiate your struct
    forms
}
