use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerUrl {
    hostname: String,
    port: Option<u16>,
    is_secure: bool,
}

impl ServerUrl {
    pub fn parse(value: &str) -> Option<Self> {
        let re = Regex::new(
            r"^(?:(?P<scheme>https?|wss?)://)?(?:www\.)?(?P<host>[^:/\s]+)(?::(?P<port>\d+))?(?:/.*)?$"
        ).ok()?;

        let caps = re.captures(value.trim())?;
        let scheme = caps.name("scheme").map(|m| m.as_str());
        let host = caps.name("host").map(|m| m.as_str())?;
        let port: Option<u16> = caps
            .name("port")
            .map(|m| m.as_str())
            .and_then(|s| s.parse().ok());

        let is_secure = scheme == Some("https") || scheme == Some("wss");

        Some(Self {
            hostname: host.to_string(),
            port,
            is_secure,
        })
    }

    pub fn is_valid(value: &str) -> bool {
        Self::parse(value).is_some()
    }

    pub fn as_ws_url(&self) -> String {
        format!(
            "{}://{}{}/ws",
            if self.is_secure { "wss" } else { "ws" },
            self.hostname,
            self.port.map(|p| format!(":{}", p)).unwrap_or_default()
        )
    }

    pub fn as_http_url(&self) -> String {
        format!(
            "{}://{}{}/ws",
            if self.is_secure { "https" } else { "http" },
            self.hostname,
            self.port.map(|p| format!(":{}", p)).unwrap_or_default()
        )
    }

    pub fn as_platform_specific_url(&self) -> String {
        #[cfg(target_arch = "wasm32")]
        return self.as_http_url();
        #[cfg(not(target_arch = "wasm32"))]
        return self.as_ws_url();
    }

    pub fn as_human_readable_url(&self) -> String {
        format!(
            "{}://{}{}",
            if self.is_secure { "https" } else { "http" },
            self.hostname,
            self.port.map(|p| format!(":{}", p)).unwrap_or_default()
        )
    }
}
