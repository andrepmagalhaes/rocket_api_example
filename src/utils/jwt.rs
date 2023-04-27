use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::{json, Json};
use rocket::Request;
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{
    OAuthFlows, Object, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use serde::{Deserialize, Serialize};

use crate::models::user::User;

static KEY: &[u8] = b"secret";
static ONE_DAY: i64 = 60 * 60 * 24;

/// JWT Claims Struct
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Claims {
    // user_id
    pub sub: i32,
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
}

/// Token Validation Struct
#[derive(Debug, JsonSchema)]
pub struct TokenValidation {
    /// JWT Claims
    pub claims: Claims,
}

/// Token Validation Implementation for route guards
#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenValidation {
    type Error = String;

    // Function that checks if the token is valid
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the token from the request header
        let token = request.headers().get_one("Authorization");

        // Check if the token exists
        match token {
            Some(token) => {
                // Remove the Bearer prefix from the token
                let token = token.replace("Bearer ", "");

                // Decode the token
                let decode_token = jsonwebtoken::decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(&KEY),
                    &jsonwebtoken::Validation::default(),
                );

                // Check if the token is valid
                let result = match decode_token {
                    Ok(token_data) => token_data,
                    Err(_) => {
                        return Outcome::Failure((
                            rocket::http::Status::Unauthorized,
                            "Invalid token".to_string(),
                        ));
                    }
                };

                // Check if the token is expired
                let now = Utc::now().timestamp_nanos() / 1_000_000_000;

                // Return an error if the token is expired
                if result.claims.exp < now {
                    return Outcome::Failure((
                        rocket::http::Status::Unauthorized,
                        "Token expired".to_string(),
                    ));
                }

                // Returns TokenValidation struct if the token is valid
                return Outcome::Success(TokenValidation {
                    claims: result.claims,
                });
            }
            None => {
                // Return an error if the token is not found
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    "Token not found".to_string(),
                ));
            }
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for TokenValidation {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some("Requires an API key to access, key is: `mykey`.".to_owned()),
            // Setup data requirements.
            // This can be part of the `header`, `query` or `cookie`.
            // In this case the header `x-api-key: mykey` needs to be set.
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("ApiKeyAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}

/// Generate JWT token function
pub fn generate_token(user_data: User) -> Result<String, String> {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000;

    let claims = Claims {
        sub: user_data.id,
        iat: now,
        exp: now + ONE_DAY,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(&KEY));

    match token {
        Ok(token) => {
            return Ok(token);
        }
        Err(_) => {
            return Err("Failed to generate token".to_string());
        }
    }
}

// Deprecated
pub fn verify_token(token: String) -> Result<(), String> {
    let decode_token = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &jsonwebtoken::Validation::default(),
    );

    let result = match decode_token {
        Ok(token_data) => token_data,
        Err(_) => {
            return Err("Failed to decode token".to_string());
        }
    };

    let now = Utc::now().timestamp_nanos() / 1_000_000_000;

    if result.claims.exp < now {
        return Err("Token expired".to_string());
    }

    return Ok(());
}
