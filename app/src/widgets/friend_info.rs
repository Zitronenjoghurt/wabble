use egui::{Response, Ui, Widget};
use wabble_core::types::friend_info::FriendInfo;

pub struct FriendInfoWidget<'a> {
    info: &'a FriendInfo,
    removed: &'a mut bool,
}

impl<'a> FriendInfoWidget<'a> {
    pub fn new(info: &'a FriendInfo, removed: &'a mut bool) -> Self {
        Self { info, removed }
    }
}

impl Widget for FriendInfoWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            let time =
                chrono::DateTime::from_timestamp(self.info.timestamp_utc, 0).unwrap_or_default();
            ui.label(format!("Friends since {}", time));
            ui.separator();
            if ui.button("End Friendship").clicked() {
                *self.removed = true;
            }
        })
        .response
    }
}
