use crate::{util::Theme, Approach};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct AppConfig {
    pub theme: Option<Theme>,
    pub font_size: Option<f64>,
    pub approach: Option<Approach>,
}
