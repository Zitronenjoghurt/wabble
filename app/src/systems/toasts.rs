use egui::Context;
use egui_notify::Toasts;

#[derive(Default)]
pub struct ToastSystem {
    toasts: Toasts,
}

impl ToastSystem {
    pub fn update(&mut self, ctx: &Context) {
        self.toasts.show(ctx);
    }

    pub fn success(&mut self, text: impl Into<String>) {
        self.toast(ToastConfig::success(text).duration(std::time::Duration::from_secs(5)));
    }

    pub fn error(&mut self, text: impl Into<String>) {
        self.toast(ToastConfig::error(text));
    }

    pub fn info(&mut self, text: impl Into<String>) {
        self.toast(ToastConfig::info(text).duration(std::time::Duration::from_secs(5)));
    }

    pub fn toast(&mut self, config: ToastConfig) {
        match config.toast_type {
            ToastType::Info => {
                self.toasts
                    .info(config.text)
                    .closable(config.closable)
                    .duration(config.duration);
            }
            ToastType::Success => {
                self.toasts
                    .success(config.text)
                    .closable(config.closable)
                    .duration(config.duration);
            }
            ToastType::Error => {
                self.toasts
                    .error(config.text)
                    .closable(config.closable)
                    .duration(config.duration);
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToastType {
    #[default]
    Info,
    Success,
    Error,
}

#[derive(Default)]
pub struct ToastConfig {
    text: String,
    duration: Option<std::time::Duration>,
    toast_type: ToastType,
    closable: bool,
}

impl ToastConfig {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            closable: true,
            ..Default::default()
        }
    }

    pub fn duration(mut self, duration: std::time::Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.toast_type = toast_type;
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self::new(text).toast_type(ToastType::Info)
    }

    pub fn success(text: impl Into<String>) -> Self {
        Self::new(text).toast_type(ToastType::Success)
    }

    pub fn error(text: impl Into<String>) -> Self {
        Self::new(text).toast_type(ToastType::Error)
    }
}
