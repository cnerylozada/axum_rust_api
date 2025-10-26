use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
