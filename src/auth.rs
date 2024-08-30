use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use rand::rngs::OsRng;

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

