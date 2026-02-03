mod config;
mod node;
mod peer;
mod gossip;
mod network;
mod observability;
mod security;

use crate::config::NodeConfig;
use crate::node::Node;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();

    let config = NodeConfig::load("config/node.toml")
        .expect("failed to load config");

    let node = Node::new(config);
    node.run().await;
}
