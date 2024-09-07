

use askama::Template;
use askama_axum::IntoResponse;
//To do: Generate forms here based on formid given by the user.
//Route based on form id
use axum::{body::Body, http::Response, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};
use crate::{jwt_auth::verify_cookie, router::to_login, DbPools};

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormsTemplate<'a> {
    id: &'a str,
}


pub fn form_router() -> Router<DbPools> {
    Router::new()
        .route("/forms", get(forms))
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn forms(cookies: Cookies) -> Response<Body> {
    let cookie_ver = verify_cookie(&cookies, "Access_token_user".to_string()).await;
    if !cookie_ver.0 {
        return to_login().await;
    }
    let forms = FormsTemplate { id: "12e4" }; // instantiate your struct
    forms.into_response()
}
