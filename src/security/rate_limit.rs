use governor::{Quota, RateLimiter as GovLimiter};
use std::num::NonZeroU32;

pub struct RateLimiter {
    _limiter: GovLimiter,
}

impl RateLimiter {
    pub fn new(per_sec: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(per_sec).unwrap());
        Self {
            _limiter: GovLimiter::direct(quota),
        }
    }
}
