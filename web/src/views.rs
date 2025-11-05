use crate::WabbleApp;
use egui::Context;
use serde::{Deserialize, Serialize};

mod main_menu;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewID {
    #[default]
    MainMenu,
}

pub trait View {
    fn update(&mut self, app: &mut crate::WabbleApp, ctx: &egui::Context);
}

#[derive(Default, Serialize, Deserialize)]
pub struct ViewManager {
    main_menu: main_menu::MainMenuView,
}

impl View for ViewManager {
    fn update(&mut self, app: &mut WabbleApp, ctx: &Context) {
        match app.current_view {
            ViewID::MainMenu => self.main_menu.update(app, ctx),
        }
    }
}
