use dotenvy::dotenv;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;
use time::{Duration, OffsetDateTime};
use tower_cookies::{Cookie, Cookies};
//use jsonwebtoken::errors::ErrorKind;

#[allow(unused)]
pub struct JWToken {
    claim: Claims,
    token: String,
}

pub enum Utype {
    User,
    Admin,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user: String,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
    is_admin: bool,
    id: String,
}

pub struct CookieClaims {
    pub is_user: bool,
    pub is_admin: bool,
}

mod jwt_numeric_date {
    // Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    // Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

impl Claims {
    /// If a token should always be equal to its representation after serializing and deserializing
    /// again, this function must be used for construction. `OffsetDateTime` contains a microsecond
    /// field but JWT timestamps are defined as UNIX timestamps (seconds). This function normalizes
    /// the timestamps.
    pub fn new(
        sub: String,
        user: String,
        iat: OffsetDateTime,
        exp: OffsetDateTime,
        is_admin: bool,
        id: String,
    ) -> Self {
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();
        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();

        Self {
            sub,
            user,
            iat,
            exp,
            is_admin,
            id,
        }
    }
}

impl JWToken {
    // Creates a new JWT Token
    // Takes inputs as email, Username, isadmin and the id
    // Currently validity of tokens is set to 1 day
    pub async fn new(email: &str, username: &str, isadmin: bool, id: &str) -> JWToken {
        dotenv().ok();
        let iat = OffsetDateTime::now_utc();
        let exp = iat + Duration::days(1);

        let claim = Claims::new(
            email.to_owned(),
            username.to_owned(),
            iat,
            exp,
            isadmin,
            id.to_string(),
        );

        let header = Header {
            kid: Some("EnvKey".to_owned()),
            alg: Algorithm::HS512,
            ..Default::default()
        };

        let token = match encode(
            &header,
            &claim,
            &EncodingKey::from_secret(
                env::var("KEY")
                    .expect("env variable KEY must be set!")
                    .as_bytes(),
            ),
        ) {
            Ok(t) => t,
            Err(_) => panic!(), // in practice you would return the error
        };
        Self { claim, token }
    }

    pub async fn all_claims(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let validation = Validation::new(Algorithm::HS512);
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(
                env::var("KEY")
                    .expect("env variable KEY must be set!")
                    .as_bytes(),
            ),
            &validation,
        )
    }

    // Performs token validations
    // checks for all necessary claims
    // Returns claims by the token (is admin or is user)
    pub async fn validate_token(token: String) -> CookieClaims {
        let jwtoken = match JWToken::all_claims(&token).await {
            Ok(c) => c,
            Err(err) => {
                println!("Parsing JWT was unsuccessful. The JWT_auth manager provided following note: {err}");
                return CookieClaims {
                    is_admin: false,
                    is_user: false,
                };
            }
        };

        CookieClaims {
            is_admin: jwtoken.claims.is_admin,

            // The is_user verification method is for time being and needs change.
            is_user: jwtoken.header.kid.expect("Unable to get key id") == *"EnvKey",
        }
    }

    // Embed token into the cookie provided
    // Takes JWT token as input along with cookie and user type data
    // Performs embedding and does not return any value.
    pub async fn embed_to_cookie(self, cookie: Cookies, utype: Utype) {
        let (name, path) = match utype {
            Utype::Admin => ("Access_token_admin", "/admin"),
            Utype::User => ("Access_token_user", "/"),
        };

        let mut auth_cookie = Cookie::new(name, self.token);
        auth_cookie.set_http_only(true);
        auth_cookie.set_secure(true);
        auth_cookie.set_path(path);
        cookie.add(auth_cookie)
    }

    //Takes cookies as inputs
    //extracts the JWT token string and validates it
    //Returns the claims of type CookieClaims
    pub async fn verify_cookie(cookies: &Cookies, utype: Utype) -> CookieClaims {
        let name = match utype {
            Utype::Admin => "Access_token_admin",
            Utype::User => "Access_token_user",
        };
        let cookie = cookies.get(name);
        if cookie.is_none() {
            CookieClaims {
                is_user: false,
                is_admin: false,
            }
        } else {
            let unpacked_cookie = cookie.expect("Unable to read cookie");
            JWToken::validate_token(unpacked_cookie.value().to_string()).await
        }
    }
}
