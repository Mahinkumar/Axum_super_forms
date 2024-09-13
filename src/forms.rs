use crate::{
    jwt_auth::{JWToken, Utype},
    mem_kv::{cache_form_input, retrieve_forms},
    router::{to_home, to_login},
    DbPools,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::{Path, State},
    http::Response,
    routing::get,
    Router,
};
use bb8_redis::redis;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use tower_cookies::{CookieManagerLayer, Cookies};
use urlencoding::decode;

#[derive(Debug, Deserialize, FromRedisValue, Serialize, ToRedisArgs)]
pub struct FormInput {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Deserialize, FromRedisValue, Serialize, ToRedisArgs)]
pub struct FormInputAll {
    pub user_id: String,
    pub uname: String,
    pub fname: String,
    pub inputs: Vec<FormInput>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormField {
    pub fid: String,
    pub typ: String,
    pub fname: String,
    pub question: String,
}

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormsTemplate<'a> {
    id: &'a str,
    el: Vec<FormField>,
}

pub fn form_router() -> Router<DbPools> {
    Router::new()
        .route("/forms/:id", get(forms).post(form_post_handler))
        .layer(CookieManagerLayer::new())
    //  .layer(TraceLayer::new_for_http()) // For Debug only
}

pub async fn forms(
    State(db_pools): State<DbPools>,
    cookies: Cookies,
    Path(form_id): Path<String>,
) -> Response<Body> {
    let cookie_ver = JWToken::verify_cookie(&cookies, Utype::User).await;
    if !cookie_ver.is_user {
        return to_login().await;
    }
    let form_fields = match retrieve_forms(&form_id, &db_pools.redis_pool).await {
        Ok(c) => c,
        Err(err) => {
            println!("{err}");
            return to_home().await;
        }
    };
    let forms = FormsTemplate {
        id: &form_id,
        el: form_fields.fields,
    }; // instantiate your struct
    forms.into_response()
}

pub async fn form_post_handler(
    State(db_pools): State<DbPools>,
    cookies: Cookies,
    Path(form_id): Path<String>,
    body: String,
) {
    let cookie_ver = JWToken::verify_cookie(&cookies, Utype::User).await;
    if !cookie_ver.is_user {
        println!("Someone not a user tried posting to the form. {form_id}")
    }
    let body = decode(&body).expect("Unable to handle the post request");
    let v: Vec<&str> = body.rsplit('&').collect();
    let mut form_inputs: Vec<FormInput> = vec![];
    for i in v {
        let kv = i.rsplit_once("=").expect("Unable to split");
        let item: FormInput = FormInput {
            name: kv.0.to_string(),
            value: kv.1.to_string(),
        };
        form_inputs.push(item);
    }
    let cookie = cookies
        .get("Access_token_user")
        .expect("Unable to read cookie");

    let claims = JWToken::all_claims(cookie.value())
        .await
        .expect("Unable to unpack cookie");

    let username = claims.claims.user;
    let inputs: FormInputAll = FormInputAll {
        user_id: claims.claims.id.clone(),
        uname: username.clone(),
        fname: form_id.clone(),
        inputs: form_inputs,
    };
    cache_form_input(&username, &claims.claims.id ,&form_id, &db_pools.redis_pool, inputs).await;
}
