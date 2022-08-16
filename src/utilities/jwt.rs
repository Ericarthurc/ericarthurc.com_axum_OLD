use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    id: String,
    exp: i64,
}

pub fn generate_jwt(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    match encode(
        &Header::default(),
        &Claims {
            id: user_id,
            exp: Utc::now().timestamp()
                + env::var("JWT_EXPIRATION").unwrap().parse::<i64>().unwrap(),
        },
        &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
    ) {
        Ok(t) => Ok(t),
        Err(err) => Err(err),
    }
}

pub fn validate_jwt(token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
        &validation,
    ) {
        Ok(c) => Ok(c),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => Err(err),
            ErrorKind::ExpiredSignature => Err(err),
            _ => Err(err),
        },
    }
}
