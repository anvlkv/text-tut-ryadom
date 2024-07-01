use color_art::Color;
use dioxus_hooks::use_context;
use dioxus_signals::{MappedSignal, Readable};

use crate::state::StateCtx;

#[derive(
    serde::Serialize,
    serde::Deserialize,
    strum::VariantArray,
    strum::Display,
    Clone,
    Copy,
    Debug,
    PartialEq,
)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn base(&self) -> Color {
        match self {
            Theme::Dark => Color::new(20, 20, 10, 1.0),
            Theme::Light => Color::new(230, 230, 240, 1.0),
        }
    }

    pub fn accent(&self) -> Color {
        match self {
            Theme::Dark => Color::new(108, 157, 129, 1.0),
            Theme::Light => Color::new(255, 168, 193, 1.0),
        }
    }

    pub fn text(&self) -> Color {
        let base = self.base();
        base.negate()
    }

    pub fn surface(&self, level: usize) -> Color {
        let base = self.base();
        match self {
            Self::Dark => base.lighten(level as f64 * 0.02),
            Self::Light => base.darken(level as f64 * 0.02),
        }
    }

    pub fn shadow(&self, elevation: usize) -> String {
        let shadow_color = self.base().negate();
        format!(
            "0 0 {} 2 rgb({}, {}, {}, 120)",
            elevation * 5,
            shadow_color.red(),
            shadow_color.green(),
            shadow_color.blue()
        )
    }
}

pub fn use_app_theme() -> MappedSignal<Theme> {
    let StateCtx(state, _) = use_context();

    state.map(|s| {
        s.config
            .as_ref()
            .map(|c| c.theme.as_ref())
            .flatten()
            .unwrap_or(&Theme::Light)
    })
}
