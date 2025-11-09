use crate::widgets::friend::FriendWidget;
use crate::windows::friend_info::FriendInfoWindowState;
use egui::ahash::HashMap;
use egui::{Grid, Response, ScrollArea, Ui, Widget};
use wabble_core::types::friend_info::FriendInfo;

pub struct FriendList<'a> {
    list: &'a HashMap<String, FriendInfo>,
    friend_info_window: &'a mut FriendInfoWindowState,
}

impl<'a> FriendList<'a> {
    pub fn new(
        list: &'a HashMap<String, FriendInfo>,
        friend_info_window: &'a mut FriendInfoWindowState,
    ) -> Self {
        Self {
            list,
            friend_info_window,
        }
    }
}

impl Widget for FriendList<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.list.is_empty() {
            ui.small("No friends")
        } else {
            ScrollArea::vertical()
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        Grid::new("friend_list_grid")
                            .num_columns(1)
                            .striped(true)
                            .show(ui, |ui| {
                                for friend in self.list.values() {
                                    FriendWidget::new(friend, self.friend_info_window).ui(ui);
                                }
                            })
                    })
                    .response
                })
                .inner
        }
    }
}
