use egui::{Response, Ui, Widget};
use egui_phosphor::regular;
use wabble_core::types::friend_info::FriendInfo;

pub struct FriendWidget<'a> {
    info: &'a FriendInfo,
}

impl<'a> FriendWidget<'a> {
    pub fn new(info: &'a FriendInfo) -> Self {
        Self { info }
    }
}

impl Widget for FriendWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let icon = if self.info.is_online {
                regular::USER_CIRCLE
            } else {
                regular::USER_CIRCLE_DASHED
            };
            ui.label(icon);
            ui.label(self.info.user_name.clone());
        })
        .response
    }
}
