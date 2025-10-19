use crate::controllers::users;
use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

pub fn user_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(users::get_user_list))
        .route("/{user_id}", get(users::get_user_by_id))
        .with_state(pool)
}
