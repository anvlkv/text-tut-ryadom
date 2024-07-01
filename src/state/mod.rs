mod config;
mod project;

pub use config::*;
pub use project::*;

use dioxus_hooks::UseCallback;
use dioxus_signals::Signal;

use crate::{resource::DataSrc, util::Theme, Approach};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct AppState {
    pub data_src: Option<DataSrc>,
    pub config: Option<AppConfig>,
    pub view: Option<ProjectView>,
}

pub enum AppStateEvent {
    SetupApproach(Approach),
    SetupFontSize(f64),
    SetupTheme(Theme),
    SetDataSource(DataSrc),
}

#[derive(Clone, Copy)]
pub struct StateCtx(pub Signal<AppState>, pub UseCallback<()>);

impl AppState {
    pub fn update(&mut self, ev: AppStateEvent) {
        match ev {
            AppStateEvent::SetupApproach(approach) => {
                let config = self.config.get_or_insert(AppConfig::default());
                config.approach = Some(approach);
            }
            AppStateEvent::SetupFontSize(size) => {
                let config = self.config.get_or_insert(AppConfig::default());
                config.font_size = Some(size);
            }
            AppStateEvent::SetupTheme(theme) => {
                let config = self.config.get_or_insert(AppConfig::default());
                config.theme = Some(theme);
            }
            AppStateEvent::SetDataSource(d) => {
                self.data_src = Some(d);
            }
        }
    }
}
