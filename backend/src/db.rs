use sqlx::{PgPool, Row};
use crate::models::{Snippet, User, CreateSnippet, UpdateSnippet, CreateUser, LoginCredentials};
use crate::errors::AppError;

pub struct DbPool(PgPool);

impl DbPool {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(DbPool(pool))
    }

    pub async fn create_user(&self, username: &str, password_hash: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query!(
            "INSERT INTO users (id, username, password_hash) VALUES (gen_random_uuid(), $1, $2) RETURNING id, username, created_at",
            username,
            password_hash
        )
        .fetch_one(&self.0)
        .await?;

        Ok(User {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        })
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query!(
            "SELECT id, username, created_at FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.0)
        .await?;

        Ok(user.map(|u| User {
            id: u.id,
            username: u.username,
            created_at: u.created_at,
        }))
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query!(
            "SELECT id, username, created_at FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.0)
        .await?;

        Ok(user.map(|u| User {
            id: u.id,
            username: u.username,
            created_at: u.created_at,
        }))
    }

    pub async fn create_snippet(
        &self,
        user_id: &str,
        title: &str,
        code: &str,
        language: &str,
        description: Option<&str>,
        tags: Option<&str>,
    ) -> Result<Snippet, sqlx::Error> {
        let snippet = sqlx::query!(
            "INSERT INTO snippets (id, user_id, title, code, language, description, tags) VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, $6) RETURNING *",
            user_id,
            title,
            code,
            language,
            description,
            tags
        )
        .fetch_one(&self.0)
        .await?;

        Ok(Snippet::from_db(snippet))
    }

    pub async fn update_snippet(
        &self,
        id: &str,
        user_id: &str,
        title: Option<&str>,
        code: Option<&str>,
        language: Option<&str>,
        description: Option<&str>,
        tags: Option<&str>,
    ) -> Result<Snippet, sqlx::Error> {
        let snippet = sqlx::query!(
            "UPDATE snippets SET title = COALESCE($1, title), code = COALESCE($2, code), language = COALESCE($3, language), description = COALESCE($4, description), tags = COALESCE($5, tags), updated_at = NOW() WHERE id = $6 AND user_id = $7 RETURNING *",
            title,
            code,
            language,
            description,
            tags,
            id,
            user_id
        )
        .fetch_one(&self.0)
        .await?;

        Ok(Snippet::from_db(snippet))
    }

    pub async fn get_snippet(&self, id: &str) -> Result<Option<Snippet>, sqlx::Error> {
        let snippet = sqlx::query!(
            "SELECT * FROM snippets WHERE id = $1",
            id
        )
        .fetch_optional(&self.0)
        .await?;

        Ok(snippet.map(|s| Snippet::from_db(s)))
    }

    pub async fn list_snippets(
        &self,
        user_id: Option<&str>,
        language: Option<&str>,
        tag: Option<&str>,
    ) -> Result<Vec<Snippet>, sqlx::Error> {
        let mut query = "SELECT * FROM snippets WHERE 1=1".to_string();
        let mut args: Vec<&str> = Vec::new();
        let mut param_index = 1;

        if let Some(uid) = user_id {
            query.push_str(&format!(" AND user_id = ${}", param_index));
            args.push(uid);
            param_index += 1;
        }

        if let Some(lang) = language {
            query.push_str(&format!(" AND language = ${}", param_index));
            args.push(lang);
            param_index += 1;
        }

        if let Some(t) = tag {
            query.push_str(&format!(" AND tags ILIKE ${}", param_index));
            args.push(&format!("%{}%", t));
            param_index += 1;
        }

        query.push_str(" ORDER BY created_at DESC");

        let mut q = sqlx::query(&query);
        for arg in args {
            q = q.bind(arg);
        }

        let rows = q.fetch_all(&self.0).await?;
        Ok(rows.into_iter().map(Snippet::from_db).collect())
    }

    pub async fn delete_snippet(&self, id: &str, user_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM snippets WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(&self.0)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}