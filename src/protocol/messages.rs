use log::{info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use bitcoin::consensus::{encode, Decodable};
use bitcoin::p2p::{self, address, message, message_network};
use std::io::BufReader;

use crate::utility::errors::NodeError;

pub async fn handshake_send_version(stream: &mut TcpStream) -> Result<(), NodeError> {
    let version_message = aux_build_version_message()?;
    let raw_message = message::RawNetworkMessage::new(
        bitcoin::Network::Bitcoin.magic(), 
        version_message
    );
    let serialized_message = encode::serialize(&raw_message);

    stream.write_all(&serialized_message).await?;
    Ok(())
}

fn aux_build_version_message() -> Result<message::NetworkMessage, NodeError> {
    let services = p2p::ServiceFlags::NONE;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let addr_recv = "127.0.0.1:8333".parse::<SocketAddr>().map_err(|e| NodeError::ConfigurationError(e.to_string()))?;
    let addr_from = "0.0.0.0:0".parse::<SocketAddr>().map_err(|e| NodeError::ConfigurationError(e.to_string()))?;
    let nonce: u64 = rand::random::<u64>();
    let user_agent = String::from("/Test:0.0.1/");
    let start_height: i32 = 0;

    Ok(message::NetworkMessage::Version(message_network::VersionMessage::new(
        services,
        timestamp as i64,
        address::Address::new(&addr_recv, services),
        address::Address::new(&addr_from, services),
        nonce,
        user_agent,
        start_height,
    )))
}

pub async fn handshake_receive_version(stream: &mut TcpStream) -> Result<message_network::VersionMessage, NodeError> {
    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&mut buffer).await.map_err(|_| NodeError::HandshakeError("Failed to read from stream".to_string()))?;
    
    if n == 0 {
        return Err(NodeError::HandshakeError("No data received".to_string()));
    }

    let mut cursor = BufReader::new(&buffer[..n]);
    let response = message::RawNetworkMessage::consensus_decode(&mut cursor)
        .map_err(|e| NodeError::HandshakeError(format!("Failed to decode Version message: {}", e)))?;

    match response.payload() {
        message::NetworkMessage::Version(msg) => Ok(msg.clone()),
        _ => Err(NodeError::HandshakeError("Received message is not a Version message".to_string())),
    }
}

pub async fn handshake_send_verack(stream: &mut TcpStream) -> Result<(), NodeError> {
    let verack_message = message::RawNetworkMessage::new(
        bitcoin::Network::Bitcoin.magic(),
        message::NetworkMessage::Verack,
    );

    let buffer = encode::serialize(&verack_message);
    stream.write_all(&buffer).await.map_err(|e| {
        NodeError::HandshakeError(format!("Failed to send verack message: {}", e))
    })?;

    Ok(())
}

pub fn log_print_ascii(message: &[u8]) {
    let command_end = message.iter().position(|&x| x == 0).unwrap_or(12) + 4;
    let command_end = command_end.min(message.len()); // Ensure command_end is within bounds
    let command = std::str::from_utf8(&message[4..command_end]).unwrap_or("").trim_matches(char::from(0));

    let payload_start = 20.min(message.len());
    let payload = &message[payload_start..];
    let payload_ascii = payload.iter().map(|&c| if c.is_ascii() && !c.is_ascii_control() { c as char } else { '.' }).collect::<String>();

    info!("Command: {}\nPayload: {}", command, payload_ascii);
}

pub fn reader_pop_message(buffer: &[u8]) -> Option<(&[u8], usize)> {
    let mut offset = 0;
    while offset + 20 <= buffer.len() {
        if buffer[offset..offset + 4] == [0xf9, 0xbe, 0xb4, 0xd9] {
            if offset + 20 > buffer.len() { return None; }
            if let Ok(header_array) = buffer[offset + 16..offset + 20].try_into() {
                let payload_size = u32::from_le_bytes(header_array) as usize;
                if offset + 20 + payload_size <= buffer.len() {
                    return Some((&buffer[offset..offset + 20 + payload_size], offset + 20 + payload_size));
                }
            }
        }
        offset += 1;
    }
    None
}