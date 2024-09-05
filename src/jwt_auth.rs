use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use time::{Duration, OffsetDateTime};
use tower_cookies::Cookies;
//use jsonwebtoken::errors::ErrorKind;

struct JWToken {
    claim: Claims,
    token: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user: String,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
    is_admin: bool,
    admin_id: String,
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
        admin_id: String,
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
            admin_id,
        }
    }
}

impl JWToken {
    pub async fn create_token(&mut self, email: &str, username: &str) -> JWToken {
        dotenv().ok();
        let iat = OffsetDateTime::now_utc();
        let exp = iat + Duration::days(1);

        // We only give is_admin as true here for testing purposes
        let claim = Claims::new(
            email.to_owned(),
            username.to_owned(),
            iat,
            exp,
            false,
            "e32rf".to_string(),
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

    pub async fn validate_token(token: String) -> (bool, bool) {
        let mut validation = Validation::new(Algorithm::HS512);

        validation.set_required_spec_claims(&["exp", "iat", "user", "sub", "is_admin", "admin_id"]);
        let jwtoken = match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(
                env::var("KEY")
                    .expect("env variable KEY must be set!")
                    .as_bytes(),
            ),
            &validation,
        ) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                _ => {
                    println!("Parsing JWT was unsuccessful. The JWT_auth manager provided following note: {err}");
                    return (false, false);
                }
            },
        };
        //println!("{:?}",token.header);
        //println!("{:?}",token.claims);
        (
            jwtoken.header.kid.expect("Unable to verify Key used") == "EnvKey",
            !jwtoken.claims.is_admin,
        )
    }
}

pub async fn verify_cookie(cookies: &Cookies) -> (bool, bool) {
    let cookie = cookies.get("access_token");
    if cookie.is_none() {
        return (false, false);
    } else {
        let unpacked_cookie = cookie.expect("Unable to read cookie");
        JWToken::validate_token(unpacked_cookie.value().to_string()).await
    }
}
