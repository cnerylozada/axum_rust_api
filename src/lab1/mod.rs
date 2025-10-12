mod models;
use models::{OriginIp, RawUser};

pub async fn get_origin_ip() -> Result<OriginIp, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let origin_ip = response.json::<OriginIp>().await?;
    Ok(origin_ip)
}

pub async fn get_users() -> Result<Vec<RawUser>, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/users").await?;
    let users = response.json::<Vec<RawUser>>().await?;
    Ok(users)
}
