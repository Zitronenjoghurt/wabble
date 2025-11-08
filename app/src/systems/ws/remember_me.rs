use crate::types::server_url::ServerUrl;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RememberMe {
    pub url: ServerUrl,
    pub id: String,
    pub token: String,
}
