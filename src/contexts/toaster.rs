use std::time::Duration;

use leptos::prelude::{RwSignal, StoredValue, Update, expect_context, provide_context};

#[derive(Clone, Copy)]
pub struct Toaster {
    pub toasts: RwSignal<Vec<Toast>>,
}

impl Toaster {
    pub fn new() -> Self {
        Self {
            toasts: RwSignal::new(vec![]),
        }
    }

    pub fn add_toast(&self, toast: Toast) {
        self.toasts.update(|v| v.push(toast));
    }

    pub fn remove_toast(&self, toast: &Toast) {
        self.toasts.update(|v| v.retain(|b| b != toast));
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ToastLevel {
    #[default]
    Normal,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ToastDuration {
    #[default]
    Short,
    Long,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Toast {
    pub message: StoredValue<String>,
    pub duration: Option<ToastDuration>,
    pub level: Option<ToastLevel>,
    pub show: RwSignal<bool>,
}

impl Toast {
    pub fn new(
        message: String,
        duration: Option<ToastDuration>,
        level: Option<ToastLevel>,
    ) -> Self {
        Self {
            message: StoredValue::new(message),
            duration,
            level,
            show: RwSignal::new(true),
        }
    }
}

impl From<ToastDuration> for Duration {
    fn from(value: ToastDuration) -> Self {
        match value {
            ToastDuration::Short => Self::from_millis(1250),
            ToastDuration::Long => Self::from_millis(2500),
        }
    }
}

pub fn provide_toaster_context() {
    provide_context(Toaster::new());
}

pub fn toast(toast: Toast) {
    expect_context::<Toaster>().add_toast(toast);
}
