use crate::windows::friend_info::FriendInfoWindowState;
use egui::{Response, Ui, Widget};
use egui_phosphor::regular;
use wabble_core::types::friend_info::FriendInfo;

pub struct FriendWidget<'a> {
    info: &'a FriendInfo,
    friend_info_window: &'a mut FriendInfoWindowState,
}

impl<'a> FriendWidget<'a> {
    pub fn new(info: &'a FriendInfo, friend_info_window: &'a mut FriendInfoWindowState) -> Self {
        Self {
            info,
            friend_info_window,
        }
    }
}

impl Widget for FriendWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let friend_info_open =
                self.friend_info_window.friend_id == Some(self.info.user_id.clone());
            if ui
                .selectable_label(friend_info_open, regular::WRENCH)
                .clicked()
            {
                if friend_info_open {
                    self.friend_info_window.friend_id = None;
                } else {
                    self.friend_info_window.friend_id = Some(self.info.user_id.clone());
                }
            }

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
