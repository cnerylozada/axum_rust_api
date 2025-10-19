use crate::controllers::users;
use axum::{Router, routing::get};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(users::get_user_list))
        .route("/users/{user_id}", get(users::get_user_by_id))
}
