use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RememberMe {
    pub url: String,
    pub id: String,
    pub token: String,
}
