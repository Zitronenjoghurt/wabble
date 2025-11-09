use crate::widgets::friend::FriendWidget;
use egui::{Grid, Response, ScrollArea, Ui, Widget};
use wabble_core::types::friend_info::FriendInfo;

pub struct FriendList<'a> {
    list: &'a [FriendInfo],
}

impl<'a> FriendList<'a> {
    pub fn new(list: &'a [FriendInfo]) -> Self {
        Self { list }
    }
}

impl Widget for FriendList<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.list.is_empty() {
            ui.small("No friends")
        } else {
            ScrollArea::vertical()
                .show(ui, |ui| {
                    Grid::new("friend_list_grid")
                        .num_columns(1)
                        .striped(true)
                        .show(ui, |ui| {
                            for friend in self.list {
                                FriendWidget::new(friend).ui(ui);
                            }
                        })
                        .response
                })
                .inner
        }
    }
}
