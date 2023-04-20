use jsonwebtoken::{EncodingKey, DecodingKey, Header};
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::models::user::User;

static KEY: &[u8] = b"secret";
static ONE_DAY: i64 = 60 * 60 * 24;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn generate_token(user_data: User) -> Result<String, String> {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000;

    let claims = Claims {
        sub: user_data.id.to_string(),
        iat: now,
        exp: now + ONE_DAY,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(&KEY));

    match token {
        Ok(token) => {
            return Ok(token);
        },
        Err(_) => {
            return Err("Failed to generate token".to_string());
        },
    }

}

pub fn verify_token(token: String) -> Result<(), String> {

    let decode_token = jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(&KEY), &jsonwebtoken::Validation::default());

    let result = match decode_token {
        Ok(token_data) => token_data,
        Err(_) => {
            return Err("Failed to decode token".to_string());
        },
    };

    let now = Utc::now().timestamp_nanos() / 1_000_000_000;

    if result.claims.exp < now {
        return Err("Token expired".to_string());
    }

    return Ok(());

}