use prometheus::{IntCounter, Registry};

pub struct Metrics {
    _registry: Registry,
    _messages: IntCounter,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();
        let messages = IntCounter::new("messages_total", "Total messages").unwrap();
        registry.register(Box::new(messages.clone())).unwrap();

        Self {
            _registry: registry,
            _messages: messages,
        }
    }
}
