use crate::views::{View, ViewID};
use crate::ws::WebsocketClient;
use eframe::{Frame, Storage};
use egui::Context;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct WabbleApp {
    pub current_view: ViewID,
    pub views: crate::views::ViewManager,
    #[serde(skip, default)]
    pub ws: WebsocketClient,
}

impl WabbleApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn update_views(&mut self, ctx: &Context) {
        let mut views = std::mem::take(&mut self.views);
        views.update(self, ctx);
        self.views = views;
    }
}

impl eframe::App for WabbleApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_views(ctx);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
