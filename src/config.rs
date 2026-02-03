use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct NodeConfig {
    pub node: NodeSettings,
    pub network: NetworkSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NodeSettings {
    pub id: String,
    pub listen_addr: String,
    pub metrics_addr: String,
    pub max_peers: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NetworkSettings {
    pub gossip_fanout: usize,
    pub rate_limit_per_sec: u32,
}

impl NodeConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}
