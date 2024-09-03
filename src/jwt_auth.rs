use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tower_cookies::Cookies;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user: String,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
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
    pub fn new(sub: String, user: String, iat: OffsetDateTime, exp: OffsetDateTime) -> Self {
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
        }
    }
}

const KEY: &[u8] = b"TheUltimateKey";

pub async fn create_token(email: &str, username: &str) -> String {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::days(1);

    let my_claims = Claims::new(email.to_owned(), username.to_owned(), iat, exp);

    let header = Header {
        kid: Some("signing_key".to_owned()),
        alg: Algorithm::HS512,
        ..Default::default()
    };

    let token = match encode(&header, &my_claims, &EncodingKey::from_secret(KEY)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    //println!("Token : {:?}",token);
    token
}

pub async fn validate_token(token: String) -> bool {
    let mut validation = Validation::new(Algorithm::HS512);
    validation.set_required_spec_claims(&["exp", "iat", "user", "sub"]);
    let token = match decode::<Claims>(&token, &DecodingKey::from_secret(KEY), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::ExpiredSignature => panic!("Signature Expired"), // Example on how to handle a specific error
            _ => panic!("{err}"),
        },
    };
    //println!("{:?}",token.header);
    //println!("{:?}",token.claims);
    token.header.kid.expect("Unable to solve Key ID") == "signing_key"
}

pub async fn verify_cookie(cookies: &Cookies) -> bool {
    let cookie = cookies.get("access_id");
    if cookie.is_none() {
        return false;
    } else {
        let unpacked_cookie = cookie.expect("Unable to read cookie");
        if validate_token(unpacked_cookie.value().to_string()).await {
            return true;
        } else {
            return false;
        }
    }
}
