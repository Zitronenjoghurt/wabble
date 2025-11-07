use crate::windows::admin::AdminWindowState;
use crate::windows::connection::ConnectionWindowState;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct WindowsSystem {
    pub admin_window: AdminWindowState,
    pub connection_window: ConnectionWindowState,
}
