use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const INPUT_ENDS: &str = ".input.csv";

#[derive(Clone, Serialize, Deserialize, TS)]
pub struct InputRecord {
    pub id: String,
    pub text: String,
}

#[tauri::command]
pub fn read_inputs(file: &str) -> Result<Vec<InputRecord>, String> {
    let mut reader = csv::Reader::from_path(file).map_err(|e| e.to_string())?;

    let mut entries: Vec<InputRecord> = Vec::new();

    for result in reader.deserialize() {
        entries.push(result.map_err(|e| e.to_string())?)
    }

    Ok(entries)
}
