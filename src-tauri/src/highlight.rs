use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const HIGHLIGHT_ENDS: &str = ".highlights.csv";

#[derive(Clone, Serialize, Deserialize, TS)]
pub struct HighlightRecord {
    pub text_id: String,
    pub group_id: String,
    pub start: usize,
    pub end: usize,
}

#[tauri::command]
pub fn read_highlights(file: &str) -> Result<Vec<HighlightRecord>, String> {
    let mut reader = csv::Reader::from_path(file).map_err(|e| e.to_string())?;

    let mut entries: Vec<HighlightRecord> = Vec::new();

    for result in reader.deserialize() {
        entries.push(result.map_err(|e| e.to_string())?)
    }

    Ok(entries)
}