mod config;
mod data_src;
mod project;

pub use config::*;
pub use data_src::*;
use dioxus_hooks::UseCallback;
use dioxus_signals::Signal;
pub use project::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct AppState {
    pub data_src: Option<DataSrc>,
    pub config: Option<AppConfig>,
    pub view: Option<ProjectView>,
}

#[derive(Clone, Copy)]
pub struct StateCtx(pub Signal<AppState>, pub UseCallback<()>);
