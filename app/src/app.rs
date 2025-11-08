use crate::systems::toasts::ToastSystem;
use crate::systems::windows::WindowsSystem;
use crate::systems::ws::{WebsocketClient, WebsocketError};
use crate::views::{View, ViewID, ViewManager};
use eframe::epaint::text::FontDefinitions;
use eframe::{Frame, Storage};
use egui::Context;
use wabble_core::message::server::{ServerError, ServerMessage};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct WabbleApp {
    pub current_view: ViewID,
    pub views: ViewManager,
    pub windows: WindowsSystem,
    #[serde(skip, default)]
    pub toasts: ToastSystem,
    pub ws: WebsocketClient,
}

impl WabbleApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self::setup_fonts(&cc.egui_ctx);

        let mut app: WabbleApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let _ = app.ws.connect_with_remember_me();
        app
    }

    fn setup_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "phosphor".into(),
            egui::FontData::from_static(egui_phosphor::Variant::Regular.font_bytes()).into(),
        );

        if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            font_keys.insert(1, "phosphor".into());
        }

        ctx.set_fonts(fonts);
    }

    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }

    fn update_views(&mut self, ctx: &Context) {
        let mut views = std::mem::take(&mut self.views);
        views.update(self, ctx);
        self.views = views;
    }

    fn update_ws(&mut self) {
        match self.ws.receive() {
            Ok(messages) => {
                for message in messages {
                    self.handle_message(message);
                }
            }
            Err(err) => {
                if !matches!(err, WebsocketError::NotConnected) {
                    self.toasts.error(err.to_string());
                }
                self.switch_view(ViewID::Login);
            }
        }
    }

    fn handle_message(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::Error(err) => {
                self.toasts.error(err.to_string());
                match err {
                    ServerError::Unauthorized | ServerError::InvalidCredentials => {
                        self.switch_view(ViewID::Login);
                    }
                    _ => {}
                }
            }
            ServerMessage::Authenticated(_) => {
                self.toasts.success("Login successful");
                self.switch_view(ViewID::Main);
            }
            ServerMessage::FriendRequestSent => {
                self.toasts.success("Friend request sent");
            }
            _ => {}
        }
    }
}

impl eframe::App for WabbleApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.update_views(ctx);
        self.update_ws();
        self.toasts.update(ctx);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
