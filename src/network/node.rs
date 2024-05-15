use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use std::fs::File;
use std::io::{BufRead, BufReader};
use log::{info, error};

use crate::network::connection;
use crate::utility::errors::NodeError;
use crate::config::Config;

pub struct Node {
    pub ip: String,
    pub port: u32,
}

pub async fn start_node(config: &Config) -> Result<(), NodeError> {
    let nodes = load_nodes(&config.nodes_path)?;

    for node in nodes.iter().cycle() {
        info!("Trying to connect to {}:{}", node.ip, node.port);
        match TcpStream::connect(format!("{}:{}", node.ip, node.port)).await {
            Ok(socket) => {
                info!("Connected to {}:{}", node.ip, node.port);
                if let Err(e) = connection::handle_connection(socket, config).await {
                    error!("Error handling connection to {}: {}: {}", node.ip, node.port, e);
                }
            }
            Err(e) => {
                error!("Failed to connect to {}:{}. Error: {}", node.ip, node.port, e);
            }
        }
        info!("Sleeping for 5 seconds before connection to another node");
        time::sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

fn load_nodes(file_path: &str) -> Result<Vec<Node>, NodeError> {
    let file = File::open(file_path).map_err(NodeError::from)?;
    let reader = BufReader::new(file);
    let mut nodes = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(NodeError::from)?;
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() == 2 {
            let ip = parts[0].to_string();
            let port = parts[1].parse::<u32>().map_err(NodeError::from)?;
            nodes.push(Node { ip, port });
        }
    }

    Ok(nodes)
}