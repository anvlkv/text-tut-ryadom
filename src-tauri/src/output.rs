use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const OUTPUT_ENDS: &str = ".output.csv";

#[derive(Clone, Serialize, Deserialize, TS)]
pub struct OutputRecord {
    pub text_id: String,
    pub text: String,
    pub completed_ts: Option<chrono::DateTime<chrono::Utc>>,
}

#[tauri::command]
pub fn write_output(file: &str, output: OutputRecord) -> Result<(), String> {
    let mut entries = read_outputs(file)?;

    entries.retain(|e| e.text_id != output.text_id);
    entries.push(output);

    let mut writer = csv::Writer::from_path(file).map_err(|e| e.to_string())?;

    for record in entries {
        writer.serialize(record).map_err(|e| e.to_string())?;
    }

    writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn read_outputs(file: &str) -> Result<Vec<OutputRecord>, String> {
    let mut reader = csv::Reader::from_path(file).map_err(|e| e.to_string())?;

    let mut entries: Vec<OutputRecord> = Vec::new();

    for result in reader.deserialize() {
        entries.push(result.map_err(|e| e.to_string())?)
    }

    Ok(entries)
}
