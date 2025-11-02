use axum::{Router, routing::get};
mod tokens;
mod users;
use sqlx::{Pool, Postgres};
use users::user_routes;

use crate::routes::tokens::tokens_routes;

pub fn main_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to Axum101!" }))
        .nest("/users", user_routes(pool.clone()))
        .nest("/tokens", tokens_routes(pool.clone()))
}
