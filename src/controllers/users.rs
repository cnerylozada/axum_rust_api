use axum::{Json, extract::Path};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: String,
    username: String,
    age: u64,
}

pub async fn get_user_list() -> Json<Vec<User>> {
    let result = vec![
        User {
            id: "0001".to_string(),
            username: "cnerylozada".to_string(),
            age: 32,
        },
        User {
            id: "0002".to_string(),
            username: "tatiana".to_string(),
            age: 31,
        },
    ];
    Json(result)
}

pub async fn get_user_by_id(Path(user_id): Path<String>) -> Json<User> {
    println!("get_user_by_id user_id: {}", user_id);

    let user = User {
        id: "0001".to_string(),
        username: "cnerylozada".to_string(),
        age: 32,
    };
    Json(user)
}
