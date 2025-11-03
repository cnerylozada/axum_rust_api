use crate::{
    controllers::models::{CreteUserDto, User, UserListQuery},
    documentation::api_tags,
    middlewares::manage_authentication,
    models::ApiErrorResponse,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
};
use sqlx::{PgPool, types::Uuid};

#[utoipa::path(
    tag = api_tags::USERS,
    get,
    path = "/api/v1/users",
    responses(
        (status = 200, description = "All registered users", body = [User]),
    ),
)]
pub async fn get_user_list(
    Query(params): Query<UserListQuery>,
    State(db_pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, ApiErrorResponse)> {
    let wallet = params.wallet;

    let user_list = if wallet.is_some() {
        sqlx::query_as::<_, User>("SELECT * FROM users where wallet = $1")
            .bind(wallet)
            .fetch_all(&db_pool)
            .await
            .map_err(|error| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse {
                        message: error.to_string(),
                    },
                )
            })?
    } else {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&db_pool)
            .await
            .map_err(|error| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse {
                        message: error.to_string(),
                    },
                )
            })?
    };
    Ok(Json(user_list))
}

#[utoipa::path(
    tag = api_tags::USERS,
    get,
    path = "/api/v1/users/{user_id}",
    params(
        ("user_id" = String, Path, description = "User database id to get User for")
    ),
    responses(
        (status = 200, description = "User found successfully", body = User),
    ),
)]
pub async fn get_user_by_id(
    headers: HeaderMap,
    Path(user_id): Path<String>,
    State(db_pool): State<PgPool>,
) -> Result<Json<User>, (StatusCode, ApiErrorResponse)> {
    let _ = manage_authentication(headers).map_err(|error| {
        (
            StatusCode::UNAUTHORIZED,
            ApiErrorResponse { message: error },
        )
    })?;

    let id = Uuid::parse_str(&user_id).map_err(|error| {
        (
            StatusCode::BAD_REQUEST,
            ApiErrorResponse {
                message: error.to_string(),
            },
        )
    })?;

    let query = "SELECT * FROM users WHERE id = $1";
    let user_response = sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(&db_pool)
        .await
        .map_err(|error| {
            (
                StatusCode::NOT_FOUND,
                ApiErrorResponse {
                    message: error.to_string(),
                },
            )
        })?;
    Ok(Json(user_response))
}

#[utoipa::path(
    tag = api_tags::USERS,
    post,
    path = "/api/v1/users",
    request_body = CreteUserDto,
    responses(
        (status = 200, description = "User created successfully", body = User)
    )
)]
pub async fn create_user(
    State(db_pool): State<PgPool>,
    Json(user_dto): Json<CreteUserDto>,
) -> Result<Json<User>, (StatusCode, ApiErrorResponse)> {
    let query = r#"
    INSERT INTO users (wallet) VALUES ($1)
    RETURNING id, wallet, email
    "#;

    let new_user = sqlx::query_as::<_, User>(query)
        .bind(user_dto.wallet)
        .fetch_one(&db_pool)
        .await
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse {
                    message: error.to_string(),
                },
            )
        })?;

    Ok(Json(new_user))
}
