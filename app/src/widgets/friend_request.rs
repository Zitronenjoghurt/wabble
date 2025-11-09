use egui::{Response, Ui, Widget};
use egui_phosphor::regular;
use wabble_core::types::friend_request_info::FriendRequestInfo;

pub struct FriendRequestWidget<'a> {
    info: &'a FriendRequestInfo,
    accepted: &'a mut bool,
    blocked: &'a mut bool,
}

impl<'a> FriendRequestWidget<'a> {
    pub fn new(info: &'a FriendRequestInfo, accepted: &'a mut bool, blocked: &'a mut bool) -> Self {
        Self {
            info,
            accepted,
            blocked,
        }
    }
}

impl Widget for FriendRequestWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            if ui.button(regular::CHECK_CIRCLE).clicked() {
                *self.accepted = true;
            }
            if ui.button(regular::X_CIRCLE).clicked() {
                *self.blocked = true;
            }
            ui.separator();
            ui.label(self.info.user_name.clone());
        })
        .response
    }
}
