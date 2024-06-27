#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub theme: Option<Theme>,
    pub font_size: Option<f32>,
    pub approach: Option<Approach>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum Approach {
    Polite,
}
