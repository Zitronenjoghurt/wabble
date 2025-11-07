use crate::WabbleApp;
use egui::Context;
use serde::{Deserialize, Serialize};

mod login;
mod main;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewID {
    #[default]
    Login,
    Main,
}

pub trait View {
    fn update(&mut self, app: &mut crate::WabbleApp, ctx: &egui::Context);
}

#[derive(Default, Serialize, Deserialize)]
pub struct ViewManager {
    login: login::LoginView,
    main: main::MainView,
}

impl View for ViewManager {
    fn update(&mut self, app: &mut WabbleApp, ctx: &Context) {
        match app.current_view {
            ViewID::Login => self.login.update(app, ctx),
            ViewID::Main => self.main.update(app, ctx),
        }
    }
}
