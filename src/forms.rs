
use askama::Template;
use askama_axum::IntoResponse;
use serde::{Serialize, Deserialize};
use crate::{jwt_auth::{JWToken, Utype}, mem_kv::retrieve_forms, router::{to_home, to_login}, DbPools};
use axum::{body::Body, extract::{Path, State}, http::Response, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};
use urlencoding::decode;

#[derive(Debug)]
#[derive(Deserialize)]
pub struct FormInput {
    pub name: String,
    pub value: String
}



#[derive(Debug, Deserialize, Serialize)]
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
        .route("/forms/:id", get(forms).post(form_post_handler))
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn forms(State(db_pools): State<DbPools>,cookies: Cookies, Path(form_id): Path<String>) -> Response<Body> {
    let cookie_ver = JWToken::verify_cookie(&cookies, Utype::User).await;
    if !cookie_ver.is_user {
        return to_login().await;
    }
    let form_fields = 
    match retrieve_forms(&form_id,&db_pools.redis_pool).await{
        Ok(c) => c,
        Err(err) => {
            println!("{err}");
            return to_home().await;
        } 
    };
    let forms = FormsTemplate { id: &form_id, el: form_fields.fields}; // instantiate your struct
    forms.into_response()
}

pub async fn form_post_handler(State(_db_pools): State<DbPools>,cookies: Cookies, Path(form_id): Path<String>,body: String){
    let cookie_ver = JWToken::verify_cookie(&cookies, Utype::User).await;
    if !cookie_ver.is_user {
        println!("Someone not a user tried posting to the form. {form_id}")
    }
    let body = decode(&body).expect("Unable to handle the post request");
    let v: Vec<&str> =  body.rsplit('&').collect();
    for i in v{
        let kv = i.rsplit_once("=").expect("Unable to split");
        let item: FormInput = FormInput {
            name: kv.0.to_string(),
            value: kv.1.to_string()
        };

        println!("{:?}",item);

    }

}
