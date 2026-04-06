use actix_web::{HttpResponse, HttpMessage};
use crate::errors::AppError;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn extract_user_id(req: &actix_web::HttpRequest) -> Result<String, AppError> {
        let auth_header = req.headers().get("Authorization")
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;
        
        let token = auth_header.to_str()
            .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid token format".to_string()))?;

        let config = req.app_data::<web::Data<crate::config::AppConfig>>()
            .ok_or_else(|| AppError::InternalServerError("App config not found".to_string()))?;

        let user_id = crate::auth::validate_token(token, &config.jwt_secret)?;
        Ok(user_id)
    }
}