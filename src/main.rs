mod gossip;
mod peer;

use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();
}