use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OriginIp {
    origin: String,
}

#[derive(Debug, Deserialize)]
pub struct RawCompany {
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct RawUser {
    pub id: u32,
    pub name: String,
    pub company: RawCompany,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub company: String,
}
