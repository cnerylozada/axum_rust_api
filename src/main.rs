mod lab1;
use lab1::{get_origin_ip, get_users};

#[tokio::main]
async fn main() {
    let result = get_origin_ip().await;
    println!("origin_ip {:?}", result.unwrap());

    let users = get_users().await;
    println!("users {:?}", users);
}
