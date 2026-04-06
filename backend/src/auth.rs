use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, DecodingKey, EncodingKey};
use chrono::{Utc, Duration};
use crate::errors::AppError;
use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_token(user_id: &str, secret: &str, expiry_seconds: u64) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now() + Duration::seconds(expiry_seconds as i64);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp() as usize,
    };

    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_token(token: &str, secret: &str) -> Result<String, AppError> {
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation
    ).map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    Ok(token_data.claims.sub)
}