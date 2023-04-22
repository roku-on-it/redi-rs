#[derive(Debug)]
pub enum RedisError {
    SetupError(String),
    CommandError(String),
    ConnectionError(String),
}

impl std::fmt::Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisError::SetupError(message) => write!(f, "SetupError: {}", message),
            RedisError::CommandError(message) => write!(f, "CommandError: {}", message),
            RedisError::ConnectionError(message) => write!(f, "ConnectionError: {}", message),
        }
    }
}

impl std::error::Error for RedisError {}