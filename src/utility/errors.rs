use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::time::SystemTimeError;

// Error types
#[derive(Debug)]
pub enum NodeError {
    ConfigError(String),
    IoError(std::io::Error),
    ConnectError(String),
    HandshakeError(String),
    ConfigurationError(String),
    TimeoutError(String),
}

// Improve readability
impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeError::ConfigError(ref err) => write!(f, "Configuration Error: {}", err),
            NodeError::IoError(ref err) => write!(f, "I/O Error: {}", err),
            NodeError::ConnectError(ref desc) => write!(f, "Connection Error: {}", desc),
            NodeError::HandshakeError(ref desc) => write!(f, "Handshake Error: {}", desc),
            NodeError::ConfigurationError(ref desc) => write!(f, "Configuration Error: {}", desc),
            NodeError::TimeoutError(ref desc) => write!(f, "Timeout Error: {}", desc),
        }
    }
}

// Implement std::error::Error
impl Error for NodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            NodeError::IoError(ref err) => Some(err),
            _ => None,
        }
    }
}

// Other implementations
impl From<std::io::Error> for NodeError {
    fn from(err: std::io::Error) -> Self {
        NodeError::IoError(err)
    }
}

impl From<ParseIntError> for NodeError {
    fn from(err: ParseIntError) -> Self {
        NodeError::ConfigurationError(err.to_string())
    }
}

impl From<SystemTimeError> for NodeError {
    fn from(err: SystemTimeError) -> Self {
        NodeError::ConfigurationError(err.to_string())
    }
}

impl From<dotenv::Error> for NodeError {
    fn from(err: dotenv::Error) -> Self {
        NodeError::ConfigError(err.to_string())
    }
}
