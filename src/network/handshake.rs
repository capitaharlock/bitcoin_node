use log::{info, error};
use tokio::net::TcpStream;

use crate::protocol::messages;
use crate::utility::errors::NodeError;

pub async fn perform_handshake(stream: &mut TcpStream, min_node_version: u32) -> Result<(), NodeError> {
    
    info!("handshake | Start by sending our version message");
    messages::handshake_send_version(stream).await.map_err(|e| {
        error!("handshake | Failed to send version message: {}", e);
        NodeError::HandshakeError(e.to_string())
    })?;

    info!("handshake | Receive version and services list");
    let version_message = messages::handshake_receive_version(stream).await.map_err(|e| {
        error!("handshake | Failed to receive version message: {}", e);
        NodeError::HandshakeError(e.to_string())
    })?;

    info!("handshake | Check if version compatibility: {}", version_message.version);
    if version_message.version < min_node_version {
        return Err(NodeError::HandshakeError("Incompatible Version".to_string()));
    }

    info!("handshake | Send verack to complete");
    messages::handshake_send_verack(stream).await.map_err(|e| {
        error!("handshake | Failed to send verack message: {}", e);
        NodeError::HandshakeError(e.to_string())
    })?;

    info!("handshake | End");
    Ok(())
}