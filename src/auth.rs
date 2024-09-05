use serde::Deserialize;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use rand::rngs::OsRng;
use tower_cookies::Cookies;
use axum::{response::Redirect, Form};
use axum::{
    http::Uri,
    response::IntoResponse,
};

use crate::{db::retrieve_admin, jwt_auth::JWToken, router::{embed_token, to_login}};

#[derive(Deserialize)]
pub struct AdminLogin {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    key: String,
}

pub async fn login_handler(_cookie: Cookies, uri: Uri, Form(login): Form<UserLogin>) -> impl IntoResponse {
    println!(
        "Form from {} Posted {} and was verified",
        uri, login.key
    );
}

pub async fn admin_login_handler(cookie: Cookies, uri: Uri, Form(login): Form<AdminLogin>) -> impl IntoResponse {
    println!(
        "Form posted from {} by {}.",
        uri, &login.email
    );
    let email_copy = login.email.clone();
    let admin_data = match retrieve_admin(login.email).await {
        Ok(c) => c,
        Err(err) => {println!("Unable to retrieve admin: {err}"); return to_login().await},
    };


    if !verify_hash(&admin_data.2,&login.password).await {
        return to_login().await;
    }

    let token = JWToken::new(&email_copy, &admin_data.1, true, &admin_data.0.to_string()).await;
    embed_token(token.return_token().await, cookie).await;
    println!("Evaluated the Admin Login");
    Redirect::to("/admin").into_response()
}

pub async fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Unable to hash password!");
    password_hash.to_string()
}

//pub async fn store_credentials(username: String,email: String,password: String){
//    
//}

pub async fn verify_hash(password_hash: &str, password: &str) -> bool {
    let parsed_hash =
        PasswordHash::new(&password_hash).expect("Recieved a String that is not password Hash");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

