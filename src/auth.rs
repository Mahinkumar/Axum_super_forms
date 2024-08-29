use serde::{Deserialize, Serialize};

use axum::{
    body::Body,
    extract::Request,
    http,
    http::Response,
    middleware::Next,
};

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user: String,
    exp: u64,
}

const KEY: &[u8] = b"TheUltiateKey";

pub fn create_token(email: &str,username: &str)-> String {
    let my_claims =
        Claims { sub: email.to_owned(), user: username.to_owned(), exp: 10000000000 };
    
    let header =
        Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };

    let token = match encode(&header, &my_claims, &EncodingKey::from_secret(KEY)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    //println!("Token : {:?}",token);
    token
}

pub fn decode_token(token: String)->Header{
    let token = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(KEY),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!(), // Example on how to handle a specific error
            _ => panic!(),
        },
    };
    token.header
    //println!("{:?}",token.header);
    //println!("{:?}",token.claims);
}

pub async fn authorization_middleware(mut req: Request, next: Next)->Response<Body>{
    let _auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    //println!("{:?}",auth_header);
    //We Do the authentication here or redirect to login page
    let response = next.run(req).await;
    response
}