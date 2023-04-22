use std::str::FromStr;

use redi_rs::RedisPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pool: RedisPool = RedisPool::from_str("localhost:6379").unwrap();
    // Or from SocketAddr
    // let mut pool = RedisPool::from(SocketAddr::from(([127, 0, 0, 1], 6379)));

    let _ = pool.max_connections(10).establish_pool().await?;

    pool.send_command("SET foo bar").await?;

    Ok(())
}
