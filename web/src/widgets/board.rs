use eframe::emath::{Pos2, Rect};
use eframe::epaint::{Color32, Vec2};
use egui::{Response, Sense, Ui, Widget};
use log::debug;
use serde::{Deserialize, Serialize};

pub struct WabbleBoardStyle {
    pub light_color: Color32,
    pub dark_color: Color32,
    pub light_color_hover: Color32,
    pub dark_color_hover: Color32,
}

impl WabbleBoardStyle {
    const DEFAULT: Self = Self {
        light_color: Color32::from_rgb(240, 217, 181),
        dark_color: Color32::from_rgb(181, 136, 99),
        light_color_hover: Color32::from_rgb(255, 255, 150),
        dark_color_hover: Color32::from_rgb(255, 255, 150),
    };
}

#[derive(Serialize, Deserialize)]
pub struct WabbleBoardUiState {
    pub scene_rect: Rect,
}

impl Default for WabbleBoardUiState {
    fn default() -> Self {
        Self {
            scene_rect: Rect::from_min_size(Pos2::ZERO, Vec2::splat(15.0)),
        }
    }
}

pub struct WabbleBoard<'a> {
    ui_state: &'a mut WabbleBoardUiState,
    style: Option<&'a WabbleBoardStyle>,
}

impl<'a> WabbleBoard<'a> {
    pub fn new(ui_state: &'a mut WabbleBoardUiState) -> Self {
        Self {
            ui_state,
            style: None,
        }
    }

    pub fn style(mut self, style: &'a WabbleBoardStyle) -> Self {
        self.style = Some(style);
        self
    }

    fn get_square_rect(&self, board_rect: Rect, row: usize, col: usize, square_size: f32) -> Rect {
        let x = board_rect.left() + col as f32 * square_size;
        let y = board_rect.top() + row as f32 * square_size;
        Rect::from_min_size(Pos2::new(x, y), Vec2::splat(square_size))
    }
}

impl Widget for WabbleBoard<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let style = self.style.unwrap_or(&WabbleBoardStyle::DEFAULT);
        let scene = egui::Scene::new().zoom_range(2.0..=500.0);

        scene
            .show(ui, &mut self.ui_state.scene_rect, |ui| {
                let board_size = 15;

                for row in 0..board_size {
                    for col in 0..board_size {
                        let square_rect = Rect::from_min_size(
                            Pos2::new(col as f32, row as f32),
                            Vec2::splat(1.0),
                        );

                        let is_light = (row + col) % 2 == 0;
                        let color = if is_light {
                            style.light_color
                        } else {
                            style.dark_color
                        };

                        ui.painter().rect_filled(square_rect, 0.0, color);

                        let response =
                            ui.interact(square_rect, ui.id().with((row, col)), Sense::click());
                        if response.hovered() {
                            let hover_color = if is_light {
                                style.light_color_hover
                            } else {
                                style.dark_color_hover
                            };
                            ui.painter().rect_filled(square_rect, 0.0, hover_color);
                        }

                        if response.clicked() {
                            debug!("Clicked square: ({}, {})", row, col);
                        }
                    }
                }
            })
            .response
    }
}
