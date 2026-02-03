use std::collections::HashSet;
use uuid::Uuid;

pub struct GossipRouter {
    fanout: usize,
    seen: HashSet<Uuid>,
}

impl GossipRouter {
    pub fn new(fanout: usize) -> Self {
        Self {
            fanout,
            seen: HashSet::new(),
        }
    }

    pub fn should_forward(&mut self, msg_id: Uuid) -> bool {
        self.seen.insert(msg_id)
    }
}
