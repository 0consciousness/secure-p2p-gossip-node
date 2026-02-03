use crate::config::NodeConfig;
use crate::peer::peer_manager::PeerManager;
use crate::gossip::router::GossipRouter;
use crate::observability::metrics::Metrics;
use crate::security::rate_limit::RateLimiter;
use tracing::info;

pub struct Node {
    _config: NodeConfig,
    _peer_manager: PeerManager,
    _gossip: GossipRouter,
    _metrics: Metrics,
    _rate_limiter: RateLimiter,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        let metrics = Metrics::new();
        let rate_limiter = RateLimiter::new(config.network.rate_limit_per_sec);

        Self {
            _peer_manager: PeerManager::new(config.node.max_peers),
            _gossip: GossipRouter::new(config.network.gossip_fanout),
            _metrics: metrics,
            _rate_limiter: rate_limiter,
            _config: config,
        }
    }

    pub async fn run(&self) {
        info!("node started");
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}
