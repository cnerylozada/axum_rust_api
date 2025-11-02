use crate::controllers::users::{self, create_user, get_user_list};
use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

pub fn user_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(get_user_list).post(create_user))
        .route("/{user_id}", get(users::get_user_by_id))
        .with_state(pool)
}
