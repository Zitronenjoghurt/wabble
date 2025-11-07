use egui::{Grid, ScrollArea, Widget};
use std::fmt::Display;

pub struct SimpleList<'a, T>
where
    T: Display,
{
    items: &'a [T],
    id: &'a str,
    max_height: f32,
}

impl<'a, T> SimpleList<'a, T>
where
    T: Display,
{
    pub fn new(items: &'a [T], id: &'a str) -> Self {
        Self {
            items,
            id,
            max_height: 100.0,
        }
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }
}

impl<'a, T> Widget for SimpleList<'a, T>
where
    T: Display,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ScrollArea::vertical()
                .max_height(self.max_height)
                .show(ui, |ui| {
                    Grid::new(self.id)
                        .num_columns(1)
                        .striped(true)
                        .show(ui, |ui| {
                            for item in self.items {
                                ui.label(item.to_string());
                                ui.end_row();
                            }
                        });
                });
        })
        .response
    }
}
