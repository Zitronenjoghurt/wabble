pub struct Timeout {
    expires_at: web_time::Instant,
    duration: web_time::Duration,
}

impl Timeout {
    pub fn new(duration: web_time::Duration) -> Self {
        Self {
            expires_at: web_time::Instant::now(),
            duration,
        }
    }

    pub fn from_secs(secs: u64) -> Self {
        Self::new(web_time::Duration::from_secs(secs))
    }

    pub fn reset(&mut self) {
        self.expires_at = web_time::Instant::now() + self.duration;
    }

    pub fn is_ongoing(&self) -> bool {
        !self.is_expired()
    }

    pub fn is_expired(&self) -> bool {
        if self.expires_at < web_time::Instant::now() {
            return true;
        }
        false
    }
}
