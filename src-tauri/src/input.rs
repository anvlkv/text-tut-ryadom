use std::fs;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const INPUT_ENDS_CSV: &str = ".input.csv";
pub const INPUT_ENDS_JSON: &str = ".input.json";

#[derive(Clone, Serialize, Deserialize, TS)]
pub struct InputRecord {
    pub id: String,
    pub text: String,
}

#[tauri::command]
pub fn read_csv_inputs(file: &str) -> Result<Vec<InputRecord>, String> {
    let mut reader = csv::Reader::from_path(file).map_err(|e| e.to_string())?;

    let mut entries: Vec<InputRecord> = Vec::new();

    for result in reader.deserialize() {
        entries.push(result.map_err(|e| e.to_string())?)
    }

    Ok(entries)
}

#[tauri::command]
pub fn read_json_inputs(file: &str) -> Result<Vec<InputRecord>, String> {
    let source = fs::read_to_string(file).map_err(|e| e.to_string())?;

    let data = jzon::parse(source.as_str()).map_err(|e| e.to_string())?;

    let mut entries: Vec<InputRecord> = Vec::new();

    // handle label studio format
    if let Some(data) = data.as_array() {
        entries.extend(data.iter().map(|entry| {
            let id = entry["id"].to_string();
            let text = if let Some(text) = entry["data"]["text"].as_str() {
                text.to_string()
            } else {
                format!("unsupported: {:?}", entry["data"])
            };
            InputRecord { id, text }
        }));
    } else {
        panic!("format not supported");
    }

    Ok(entries)
}
