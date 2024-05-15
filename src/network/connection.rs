use log::error;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::time::{self, Duration};

use crate::utility::errors::NodeError;
use crate::network::handshake;
use crate::protocol::messages;
use crate::config::Config;

pub async fn handle_connection(mut stream: TcpStream, config: &Config) -> Result<(), NodeError> {
    
    if let Err(e) = handshake::perform_handshake(&mut stream, config.min_node_version).await {
        error!("Handshake failed: {}", e);
        return Err(e.into());
    }

    read_loop(&mut stream).await.map_err(|e| {
        error!("Failed to read messages: {}", e);
        e.into()
    })
}

async fn read_loop(stream: &mut TcpStream) -> Result<(), NodeError> {
    let mut data_buffer = Vec::new();
    let mut read_buffer = vec![0u8; 4096];
    let timeout_duration = Duration::from_secs(30);

    loop {
        match time::timeout(timeout_duration, stream.read(&mut read_buffer)).await {
            Ok(Ok(n)) if n == 0 => {
                if !data_buffer.is_empty() {
                    messages::log_print_ascii(&data_buffer);
                }
                return Err(NodeError::ConnectError("Connection closed by peer".to_string()));
            },
            Ok(Ok(n)) => {
                data_buffer.extend_from_slice(&read_buffer[..n]);

                while let Some((message, end_index)) = messages::reader_pop_message(&data_buffer) {
                    messages::log_print_ascii(message);
                    data_buffer.drain(0..end_index);
                }
            },
            Ok(Err(e)) => return Err(NodeError::IoError(e)),
            Err(_) => return Err(NodeError::TimeoutError("Read operation timed out".to_string())),
        }
    }
}