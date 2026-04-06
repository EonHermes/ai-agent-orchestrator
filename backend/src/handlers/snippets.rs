use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::models::{ApiResponse, CreateSnippet, UpdateSnippet, Snippet};
use crate::db::DbPool;
use crate::search::SearchIndex;
use crate::auth::{validate_token, Claims};
use crate::errors::AppError;

fn extract_user_id(req: &HttpRequest) -> Result<String, AppError> {
    let auth_header = req.headers().get("Authorization")
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;
    
    let token = auth_header.to_str()
        .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid token format".to_string()))?;

    let config = req.app_data::<web::Data<crate::config::AppConfig>>()
        .ok_or_else(|| AppError::InternalServerError("App config not found".to_string()))?;

    let user_id = validate_token(token, &config.jwt_secret)?;
    Ok(user_id)
}

pub async fn create_snippet(
    pool: web::Data<DbPool>,
    search_index: web::Data<SearchIndex>,
    req: HttpRequest,
    snippet_data: web::Json<CreateSnippet>,
) -> Result<HttpResponse, AppError> {
    let user_id = extract_user_id(&req)?;

    let snippet = pool.create_snippet(
        &user_id,
        &snippet_data.title,
        &snippet_data.code,
        &snippet_data.language,
        snippet_data.description.as_deref(),
        snippet_data.tags.as_deref(),
    ).await?;

    // Index the snippet for search
    let _ = search_index.index_snippet(&snippet);

    Ok(HttpResponse::Created().json(ApiResponse::success(snippet)))
}

pub async fn get_snippet(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let snippet = pool.get_snippet(&id).await?
        .ok_or_else(|| AppError::NotFound("Snippet not found".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(snippet)))
}

pub async fn list_snippets(
    pool: web::Data<DbPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let user_id = query.get("user_id").map(String::as_str);
    let language = query.get("language").map(String::as_str);
    let tag = query.get("tag").map(String::as_str);

    let snippets = pool.list_snippets(user_id, language, tag).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(snippets)))
}

pub async fn update_snippet(
    pool: web::Data<DbPool>,
    search_index: web::Data<SearchIndex>,
    req: HttpRequest,
    path: web::Path<String>,
    update_data: web::Json<UpdateSnippet>,
) -> Result<HttpResponse, AppError> {
    let user_id = extract_user_id(&req)?;
    let id = path.into_inner();

    // First get the existing snippet to check ownership and for search index update
    let existing = pool.get_snippet(&id).await?
        .ok_or_else(|| AppError::NotFound("Snippet not found".to_string()))?;

    if existing.user_id != user_id {
        return Err(AppError::Forbidden("Cannot update snippet owned by another user".to_string()));
    }

    // Update the snippet
    let updated = pool.update_snippet(
        &id,
        &user_id,
        update_data.title.as_deref(),
        update_data.code.as_deref(),
        update_data.language.as_deref(),
        update_data.description.as_deref(),
        update_data.tags.as_deref(),
    ).await?;

    // Re-index the updated snippet
    let _ = search_index.remove_snippet(&id);
    let _ = search_index.index_snippet(&updated);

    Ok(HttpResponse::Ok().json(ApiResponse::success(updated)))
}

pub async fn delete_snippet(
    pool: web::Data<DbPool>,
    search_index: web::Data<SearchIndex>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = extract_user_id(&req)?;
    let id = path.into_inner();

    // Check ownership first
    let existing = pool.get_snippet(&id).await?
        .ok_or_else(|| AppError::NotFound("Snippet not found".to_string()))?;

    if existing.user_id != user_id {
        return Err(AppError::Forbidden("Cannot delete snippet owned by another user".to_string()));
    }

    let deleted = pool.delete_snippet(&id, &user_id).await?;

    if deleted {
        let _ = search_index.remove_snippet(&id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound("Snippet not found".to_string()))
    }
}

pub async fn search_snippets(
    pool: web::Data<DbPool>,
    search_index: web::Data<SearchIndex>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let q = query.get("q")
        .ok_or_else(|| AppError::BadRequest("Search query 'q' is required".to_string()))?;

    let top_k = query.get("limit")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);

    let results = search_index.search(q, top_k)
        .map_err(|e| AppError::InternalServerError(format!("Search failed: {}", e)))?;

    // Fetch full snippet details for each result
    let mut snippets = Vec::new();
    for (_, snippet_id) in results {
        if let Some(snippet) = pool.get_snippet(&snippet_id).await? {
            snippets.push(snippet);
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(snippets)))
}