
use askama::Template;
use askama_axum::IntoResponse;
use crate::{db::get_form_fields, jwt_auth::verify_cookie, router::to_login, DbPools};
use axum::{body::Body, extract::State, http::Response, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};

pub struct FormField{
    pub fid: String,
    pub typ: String,
    pub fname: String,
    pub question: String,
}

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormsTemplate<'a> {
    id: &'a str,
    el: Vec<FormField>
}


pub fn form_router() -> Router<DbPools> {
    Router::new()
        .route("/forms", get(forms))
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn forms(State(db_pools): State<DbPools>,cookies: Cookies) -> Response<Body> {
    let form_id = "0d00".to_string();
    let cookie_ver = verify_cookie(&cookies, "Access_token_user".to_string()).await;
    if !cookie_ver.is_user {
        return to_login().await;
    }
    let form_fields = get_form_fields(&db_pools.postgres_pool, &form_id);
    let forms = FormsTemplate { id: &form_id, el: form_fields.await }; // instantiate your struct
    forms.into_response()
}
