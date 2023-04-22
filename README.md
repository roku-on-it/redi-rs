<img src="https://repository-images.githubusercontent.com/631322166/500b55b1-f091-4be7-86db-2e7cc679a5eb" width="400" alt="redirs Logo"/>

# redi-rs (or redirs)

### redi-rs is a Lightweight Redis client with connection pooling written in Rust and 100% memory safe

redi-rs is a Redis client written in Rust. It is a lightweight client with connection pooling and 100% memory safe.
You can use it in your Rust project to connect to Redis and perform operations.

## Features

- [x] Connection pooling

# To Do
- [ ] TLS support
- [ ] Redis Cluster support
- [ ] Redis Streams support
- [ ] Redis Pub/Sub support

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
redi-rs = "0.1.0-alpha.0"
```

## Example

```rust
use std::net::SocketAddr;
use std::str::FromStr;

use redi_rs::RedisPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pool = RedisPool::from_str("localhost:6379").unwrap();
    // Or from SocketAddr
    // let mut pool = RedisPool::from(SocketAddr::from(([127, 0, 0, 1], 6379)));

    pool.max_connections(10).establish_pool().await?;

    pool.send_command("SET foo bar").await?;

    Ok(())
}
```

More examples can be found in the [examples](examples) directory.

## License

redi-rs is licensed under the MIT license. See [LICENSE](LICENSE) for more information.

## Contributing

Open an issue or a pull request.