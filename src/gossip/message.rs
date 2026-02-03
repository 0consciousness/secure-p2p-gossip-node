use uuid::Uuid;

#[derive(Clone)]
pub struct GossipMessage {
    pub id: Uuid,
    pub payload: Vec<u8>,
    pub ttl: u8,
}

impl GossipMessage {
    pub fn new(payload: Vec<u8>) -> Self {
        Self {
            id: Uuid::new_v4(),
            payload,
            ttl: 5,
        }
    }
}
