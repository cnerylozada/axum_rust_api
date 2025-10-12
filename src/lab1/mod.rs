use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OriginIp {
    origin: String,
}

pub async fn get_origin_ip() -> Result<OriginIp, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let origin_ip = response.json::<OriginIp>().await?;
    Ok(origin_ip)
}
