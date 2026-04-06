use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snippet {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub code: String,
    pub language: String,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSnippet {
    pub title: String,
    pub code: String,
    pub language: String,
    pub description: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSnippet {
    pub title: Option<String>,
    pub code: Option<String>,
    pub language: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

impl Snippet {
    pub fn from_db(row: sqlx::postgres::PgRow) -> Self {
        Snippet {
            id: row.get("id"),
            user_id: row.get("user_id"),
            title: row.get("title"),
            code: row.get("code"),
            language: row.get("language"),
            description: row.get("description"),
            tags: row.get("tags"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}