use axum::{Router, routing::get};
mod users;
use sqlx::{Pool, Postgres};
use users::user_routes;

pub fn main_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to Axum101!" }))
        .nest("/users", user_routes(pool))
}
