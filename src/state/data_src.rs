#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum DataSrc {
    Local(String),
}
