//! Lightweight, memory-safe and async Redis client with connection pooling
//!
//! # Examples
//!
//! ```
//! use std::str::FromStr;
//!
//! use redi_rs::RedisPool;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let mut pool = RedisPool::from_str("localhost:6379").unwrap();
//!
//!    let _ = pool.max_connections(10).establish_pool().await?;
//!
//!    pool.send_command("SET foo bar").await?;
//!
//!    Ok(())
//! }
//! ```

use std::cell::RefCell;
use std::collections::LinkedList;
use std::net::{SocketAddr, ToSocketAddrs};
use std::rc::Rc;
use std::str::FromStr;

pub use crate::redis_connection::RedisConnection;
use crate::redis_error::RedisError;

mod redis_connection;
mod redis_error;

pub struct RedisPool {
    max_size: u32,
    addr: SocketAddr,
    established: bool,
    // Using a linked list because it allows pushing and popping elements at either end in O(1) time.
    connections: LinkedList<Rc<RefCell<RedisConnection>>>,
}

impl RedisPool {
    pub fn max_connections(&mut self, size: u32) -> &mut Self {
        self.max_size = size;

        self
    }

    pub async fn establish_pool(&mut self) -> Result<(), RedisError> {
        if self.established {
            panic!("Redis connection pool already established, cannot establish again");
        }

        for _ in 0..self.max_size {
            let connection = RedisConnection::new(self.addr);
            match connection.await {
                Ok(connection) => {
                    self.connections
                        .push_back(Rc::new(RefCell::new(connection)));
                }
                Err(_) => eprintln!("Failed to build Redis connection pool"),
            }
        }

        self.established = true;
        Ok(())
    }

    pub async fn send_command(
        &mut self,
        command: &str,
    ) -> Result<Rc<RefCell<RedisConnection>>, RedisError> {
        match self.connections.pop_back() {
            Some(connection) => {
                let mut connection_guard = connection.borrow_mut();

                connection_guard.send_command(command).await?;

                self.connections.push_back(Rc::clone(&connection));

                Ok(Rc::clone(&connection))
            }
            None => {
                let mut connection = RedisConnection::new(self.addr).await.unwrap();

                connection.send_command(command).await?;

                Ok(Rc::new(RefCell::new(connection)))
            }
        }
    }
}

impl FromStr for RedisPool {
    type Err = RedisError;

    fn from_str(connection_str: &str) -> Result<Self, Self::Err> {
        let socket_addr = connection_str.to_socket_addrs();

        println!("socket_addr: {:?}", socket_addr);

        if let Ok(addr) = socket_addr {
            Ok(RedisPool {
                connections: LinkedList::new(),
                established: false,
                max_size: 10,
                addr: addr.collect::<Vec<SocketAddr>>()[0],
            })
        } else {
            Err(RedisError::SetupError(
                "Failed to parse connection string".to_string(),
            ))
        }
    }
}

impl From<SocketAddr> for RedisPool {
    fn from(addr: SocketAddr) -> Self {
        RedisPool {
            connections: LinkedList::new(),
            established: false,
            max_size: 10,
            addr,
        }
    }
}
