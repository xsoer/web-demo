use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::core::consts;
use crate::util::{LibResult, error::LibError};


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: String,
    pub iat: i64,
    pub exp: i64,
}

pub struct DecodeResult {
    pub id: String,
    pub is_exp: bool,
}

pub fn encode_token(id: String) -> LibResult<String> {
    let iat = chrono::Local::now().timestamp();
    let exp = iat + consts::JWT_LIVE;
    let claims = Claims { id, iat, exp };
    let header = Header {
        kid: Some(consts::JWT_KID.to_string()),
        alg: Algorithm::HS512,
        ..Default::default()
    };
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(&consts::JWT_SECRET.as_bytes()),
    )?;
    Ok(token)
}

pub fn decode_token(token: String) -> LibResult<DecodeResult> {
    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&consts::JWT_SECRET.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => c,
        Err(err) => return Err(LibError::JWTokenErr(err)),
    };

    let now = chrono::Local::now().timestamp();
    let claims = token_data.claims;
    let is_exp = if claims.exp - now < consts::JWT_EXPT { true } else { false };
    let result = DecodeResult {
        id: claims.id,
        is_exp,
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_token() {
        // dotenvy::dotenv().ok();
        let id = "0xabc".to_string();
        encode_token(id).unwrap();
    }

    #[test]
    fn test_decode_token() {
        // let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJpZCI6IjB4ZmFzZGZxd2VmYXNkZmFzZGYiLCJleHAiOjIzNDIzNH0.64iQhVtFRb5ITM2fQUmqEYuxb3WnyQIFbGg2lYIDbURPXjIuHhov1elhrwOJWA2K3QrsSXE8QstYehc2GBR__Q";
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJpZCI6IjB4ZmFzZGZxd2VmYXNkZmFzZGYiLCJleHAiOjE2OTI4OTI4MDB9.iXxls7dgXS10NnUdIKza0oETKU-zGLmliC4U5KtX0pjLyMKWP13T1L31FJ8hR5MH0MFJQWuhABFu9vqiWelc6Q";
        decode_token(token.into()).unwrap();
    }
}
