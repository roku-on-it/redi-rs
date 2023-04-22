use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::redis_error::RedisError;
use crate::redis_error::RedisError::{CommandError, ConnectionError};

pub struct RedisConnection {
    pub stream: TcpStream,
}

impl RedisConnection {
    /// Creates a new RedisConnection instance.
    ///
    /// # Arguments
    ///
    /// * [addr](SocketAddr) - The address of the Redis instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::SocketAddr;
    /// use std::str::FromStr;
    ///
    /// use redi_rs::RedisConnection;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///    let addr = SocketAddr::from_str("localhost:6379").unwrap();
    ///
    ///   let mut redis_connection = RedisConnection::new(addr).await?;
    ///
    ///  Ok(())
    /// }
    pub async fn new(addr: SocketAddr) -> Result<Self, RedisError> {
        let stream = TcpStream::connect(addr);

        match stream.await {
            Ok(stream) => Ok(RedisConnection { stream }),
            Err(_) => Err(ConnectionError("Failed to connect to Redis instance".to_string())),
        }
    }

    /// Sends a command to the Redis instance.
    ///
    /// # Arguments
    ///
    /// * [command](str) - The command to send to the Redis instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::SocketAddr;
    /// use std::str::FromStr;
    ///
    /// use redi_rs::RedisConnection;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///    let addr = SocketAddr::from_str("localhost:6379").unwrap();
    ///
    ///  let mut redis_connection = RedisConnection::new(addr).await?;
    ///
    /// redis_connection.send_command("SET foo bar").await?;
    ///
    /// Ok(())
    ///}
    /// ```
    /// Returns a [RedisConnection](RedisConnection) instance.
    pub async fn send_command(&mut self, command: &str) -> Result<&mut RedisConnection, RedisError> {
        let command = format!("{}\r\n", command);

        if let Err(why) = self.stream.write_all(command.as_bytes()).await {
            return Err(CommandError(why.to_string()));
        }

        let first_byte = &self
            .stream
            .read_u8()
            .await
            .expect("Failed to read first byte");

        match first_byte {
            b'-' => {
                let mut buf = [0; u8::MAX as usize];

                self.stream
                    .read(&mut buf)
                    .await
                    .expect("Failed to read error message");

                let redis_error_message = String::from_utf8_lossy(&buf)
                    .to_string()
                    .trim_end_matches("\0")
                    .trim_end_matches("\r\n")
                    .trim_end()
                    .to_string();

                Err(CommandError(redis_error_message))
            }
            _ => Ok(self),
        }
    }
}
