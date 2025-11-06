use anyhow::Context;
use std::sync::Arc;

pub struct Config {
    pub admin_token: String,
    pub db_url: String,
    pub ws_only: bool,
}

impl Config {
    pub fn load_from_env() -> anyhow::Result<Arc<Self>> {
        let admin_token = std::env::var("ADMIN_TOKEN").context("ADMIN_TOKEN is not set")?;
        let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
        let ws_only = std::env::var("WS_ONLY")
            .context("WS_ONLY is not set")?
            .parse()
            .context("Failed to parse WS_ONLY")?;

        Ok(Arc::new(Self {
            admin_token,
            db_url,
            ws_only,
        }))
    }
}
