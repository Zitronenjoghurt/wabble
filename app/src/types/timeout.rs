pub struct Timeout {
    created: web_time::Instant,
    duration: web_time::Duration,
}

impl Timeout {
    pub fn new(duration: web_time::Duration) -> Self {
        Self {
            created: web_time::Instant::now() - duration,
            duration,
        }
    }

    pub fn from_secs(secs: u64) -> Self {
        Self::new(web_time::Duration::from_secs(secs))
    }

    pub fn reset(&mut self) {
        self.created = web_time::Instant::now();
    }

    pub fn is_ongoing(&self) -> bool {
        !self.is_expired()
    }

    pub fn is_expired(&self) -> bool {
        if self.created.elapsed() > self.duration {
            return true;
        }
        false
    }
}
