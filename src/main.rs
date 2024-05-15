use log::{error, info};

mod config;
mod utility;
mod network;
mod protocol;

extern crate bitcoin;

use utility::errors::NodeError;

#[tokio::main]
async fn main() -> Result<(), NodeError> {
    utility::logger::setup_logger().expect("Failed to initialize logger.");

    let config = match config::Config::load() {
        Ok(cfg) => {
            info!("Configuration loaded successfully.");
            cfg
        },
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(NodeError::ConfigError(e.to_string()));
        }
    };

    match network::start_node(&config).await {
        Ok(_) => info!("Node started successfully."),
        Err(e) => {
            error!("Failed to start node: {}", e);
            return Err(NodeError::from(e));
        }
    }

    Ok(())
}
