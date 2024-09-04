use serde::Deserialize;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use rand::rngs::OsRng;
use tower_cookies::Cookies;
use axum::Form;
use axum::{
    http::Uri,
    response::IntoResponse,
};

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

pub async fn admin_login_handler(_cookie: Cookies, uri: Uri, Form(login): Form<AdminLogin>) -> impl IntoResponse {
    println!(
        "Form from {} Posted {} and Password hash was generated",
        uri, login.email
    );
    println!(
        "The Generated hash is => {}", hash_password(login.password.as_bytes()).await
    );
    
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

