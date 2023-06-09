use std::env;

use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use chrono::Utc;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}

impl JwToken {
    pub fn get_key() -> String {
        env::var("SECRET_KEY").expect("SECRET_KEY environment variable is not defined!")
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        encode(&Header::default(), &self, &key).unwrap()
    }

    pub fn new(user_id: i32) -> Self {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES").unwrap().as_i64().unwrap();
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(minutes))
            .expect("valid timestamp")
            .timestamp();
        Self {
            user_id,
            exp: expiration as usize,
        }
    }

    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let token_result = decode::<JwToken>(&token, &key, &Validation::new(Algorithm::HS256));
        match token_result {
            Ok(data) => Ok(data.claims),
            Err(error) => {
                let message = format!("{}", error);
                Err(message)
            }
        }
    }
}

impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwToken::from_token(raw_token);
                match token_result {
                    Ok(token) => ok(token),
                    Err(message) => {
                        if message == *"ExpiredSignature" {
                            return err(ErrorUnauthorized("token expired"));
                        }
                        err(ErrorUnauthorized("token can't be decoded"))
                    }
                }
            }
            None => {
                let error = ErrorUnauthorized("token not in header under key 'token'");
                err(error)
            }
        }
    }
}
