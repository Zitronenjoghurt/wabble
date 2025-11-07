#[derive(Default)]
pub struct WsStore {
    pub invite_codes: Vec<String>,
}

impl WsStore {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
