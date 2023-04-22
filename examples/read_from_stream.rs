use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use tokio::io::AsyncReadExt;

use redi_rs::{RedisConnection, RedisPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pool = RedisPool::from_str("localhost:6379").unwrap();
    // Or from SocketAddr
    // let mut pool = RedisPool::from(SocketAddr::from(([127, 0, 0, 1], 6379)));

    let _ = pool.max_connections(10).establish_pool().await?;

    let redis_connection: Rc<RefCell<RedisConnection>> = pool.send_command("SET foo bar").await?;

    let mut redis_connection_guard = redis_connection.borrow_mut();

    let mut buffer = [0; 1024];

    redis_connection_guard.stream.read(&mut buffer).await?;

    println!("Response: {}", String::from_utf8_lossy(&buffer)); // Response: +OK

    let redis_connection: Rc<RefCell<RedisConnection>> = pool.send_command("PING").await?;

    let mut redis_connection_guard = redis_connection.borrow_mut();

    let mut buffer = [0; 1024];

    redis_connection_guard.stream.read(&mut buffer).await?;

    println!("Response: {}", String::from_utf8_lossy(&buffer)); // Response: +PONG

    Ok(())
}
