mod models;
use models::{OriginIp, RawUser, User};

const JSONPLACEHOLDER_API: &str = "https://jsonplaceholder.typicode.com";

pub async fn get_origin_ip() -> Result<OriginIp, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let origin_ip = response.json::<OriginIp>().await?;
    Ok(origin_ip)
}

pub async fn get_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let response = reqwest::get(JSONPLACEHOLDER_API.to_string() + "/users").await?;
    let raw_user_list = response.json::<Vec<RawUser>>().await?;
    let users = raw_user_list
        .into_iter()
        .map(|raw_user| -> User {
            let RawUser { id, name, company } = raw_user;
            return User {
                id: id,
                name: name,
                company: company.name,
            };
        })
        .collect::<Vec<User>>();

    Ok(users)
}
