mod lab1;
use lab1::get_origin_ip;

#[tokio::main]
async fn main() {
    let result = get_origin_ip().await;
    println!("origin_ip {:?}", result.unwrap())
}
