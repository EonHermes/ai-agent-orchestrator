use actix_web::{web, HttpResponse, Responder};
use crate::models::{ApiResponse, CreateUser, User};
use crate::db::DbPool;
use crate::auth::create_token;
use crate::errors::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, AppError> {
    // Validate input
    if user_data.username.is_empty() || user_data.password.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<User>::error("Username and password required")));
    }

    // Check if user exists
    if let Some(_) = pool.get_user_by_username(&user_data.username).await? {
        return Ok(HttpResponse::Conflict().json(ApiResponse::<User>::error("Username already exists")));
    }

    // Hash password
    let password_hash = hash(&user_data.password, DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = pool.create_user(&user_data.username, &password_hash).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(user)))
}

pub async fn login(
    pool: web::Data<DbPool>,
    config: web::Data<crate::config::AppConfig>,
    credentials: web::Json<crate::models::LoginCredentials>,
) -> Result<HttpResponse, AppError> {
    let user = match pool.get_user_by_username(&credentials.username).await? {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().json(ApiResponse::error("Invalid credentials"))),
    };

    let password_hash_result = sqlx::query!("SELECT password_hash FROM users WHERE id = ?", user.id)
        .fetch_optional(&pool.0)
        .await?;

    let password_hash = match password_hash_result {
        Some(row) => row.password_hash,
        None => return Ok(HttpResponse::Unauthorized().json(ApiResponse::error("Invalid credentials"))),
    };

    verify(&credentials.password, &password_hash)
        .map_err(|e| AppError::InternalServerError(format!("Failed to verify password: {}", e)))?;

    let token = create_token(&user.id, &config.jwt_secret, config.jwt_expiry_seconds)
        .map_err(|e| AppError::InternalServerError(format!("Failed to create token: {}", e)))?;

    #[derive(Serialize)]
    struct LoginResponse {
        token: String,
        user: User,
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(LoginResponse { token, user })))
}