use crate::widgets::toggle_button::ToggleButton;
use egui::{Context, Id, Ui, Widget, WidgetText};

pub mod admin;
pub mod connection;
pub mod profile;
pub mod send_friend_request;

pub trait AppWindow: Sized {
    fn id() -> Id;
    fn title() -> impl Into<WidgetText>;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn render_content(&mut self, ui: &mut Ui);

    fn resizable(&self) -> bool {
        true
    }

    fn movable(&self) -> bool {
        true
    }

    fn collapsible(&self) -> bool {
        false
    }

    fn show(mut self, ctx: &Context) {
        let mut is_open = self.is_open();
        egui::Window::new(Self::title())
            .id(Self::id())
            .open(&mut is_open)
            .fade_in(true)
            .fade_out(true)
            .resizable(self.resizable())
            .movable(self.movable())
            .collapsible(self.collapsible())
            .show(ctx, |ui| self.render_content(ui));
        self.set_open(is_open);
    }
}

pub trait ToggleableWindow: AppWindow {
    fn toggle_label(&self) -> &'static str;

    fn toggle_button(mut self, ui: &mut Ui) -> Self {
        let mut is_open = self.is_open();
        ToggleButton::new(&mut is_open, self.toggle_label()).ui(ui);
        self.set_open(is_open);
        self
    }
}
