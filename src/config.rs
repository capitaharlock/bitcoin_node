use crate::utility::errors::NodeError;
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub nodes_path: String,
    pub min_node_version: u32,
}

impl Config {
    pub fn load() -> Result<Self, NodeError> {
        dotenv().map_err(NodeError::from)?;

        let nodes_path = env::var("NODES_PATH").unwrap_or_else(|_| "./data/nodes.csv".to_string());

        let min_node_version = env::var("MIN_NODE_VERSION")
            .unwrap_or_else(|_| "70001".to_string())
            .parse()
            .map_err(NodeError::from)?;

        Ok(Config {
            nodes_path,
            min_node_version,
        })
    }
}
