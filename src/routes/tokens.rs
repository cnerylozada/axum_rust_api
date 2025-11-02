use crate::controllers::tokens::{get_token_by_id, get_token_list};
use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

pub fn tokens_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(get_token_list))
        .route("/{token_id}", get(get_token_by_id))
        .with_state(pool)
}
