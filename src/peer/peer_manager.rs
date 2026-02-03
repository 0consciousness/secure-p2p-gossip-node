use dashmap::DashMap;

#[derive(Clone)]
pub struct PeerManager {
    peers: DashMap<String, i32>,
    max_peers: usize,
}

impl PeerManager {
    pub fn new(max_peers: usize) -> Self {
        Self {
            peers: DashMap::new(),
            max_peers,
        }
    }

    pub fn add_peer(&self, peer_id: String) {
        if self.peers.len() < self.max_peers {
            self.peers.insert(peer_id, 0);
        }
    }
}
