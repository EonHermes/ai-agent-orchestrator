use actix_web::web;

pub mod auth;
pub mod snippets;

use auth::{register, login};
use snippets::{create_snippet, get_snippet, list_snippets, update_snippet, delete_snippet, search_snippets};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/auth/register", web::post().to(register))
            .route("/auth/login", web::post().to(login))
            .route("/snippets", web::post().to(create_snippet))
            .route("/snippets", web::get().to(list_snippets))
            .route("/snippets/{id}", web::get().to(get_snippet))
            .route("/snippets/{id}", web::put().to(update_snippet))
            .route("/snippets/{id}", web::delete().to(delete_snippet))
            .route("/snippets/search", web::get().to(search_snippets))
    );
}