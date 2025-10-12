use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OriginIp {
    origin: String,
}

#[derive(Debug, Deserialize)]
struct RawCompany {
    name: String,
}
#[derive(Debug, Deserialize)]
pub struct RawUser {
    id: u32,
    name: String,
    company: RawCompany,
}

#[derive(Debug, Deserialize)]
struct User {
    id: u32,
    name: String,
    company: String,
}
