use std::sync::Arc;

pub struct Config {
    pub admin_token: String,
    pub db_url: String,
}

impl Config {
    pub fn load_from_env() -> anyhow::Result<Arc<Self>> {
        let admin_token = std::env::var("ADMIN_TOKEN")?;
        let db_url = std::env::var("DATABASE_URL")?;
        Ok(Arc::new(Self {
            admin_token,
            db_url,
        }))
    }
}
