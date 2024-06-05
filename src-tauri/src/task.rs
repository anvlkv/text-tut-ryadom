use std::fs;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    highlight::{
        read_csv_highlights, read_json_highlights, write_csv_highlights, HighlightRecord,
        HIGHLIGHT_ENDS_CSV, HIGHLIGHT_ENDS_JSON,
    },
    input::{read_csv_inputs, read_json_inputs, InputRecord, INPUT_ENDS_CSV, INPUT_ENDS_JSON},
    output::{
        read_csv_outputs, read_json_outputs, write_csv_output, OutputRecord, OUTPUT_ENDS_CSV,
        OUTPUT_ENDS_JSON,
    },
};

#[derive(Clone, Serialize, Deserialize, TS)]
pub struct Task {
    pub input: InputRecord,
    pub output: Option<OutputRecord>,
    pub highlights: Vec<HighlightRecord>,
    pub origin: String,
    pub postponed: Option<chrono::DateTime<chrono::Utc>>,
}

#[tauri::command]
pub fn read_csv_file_tasks(path: &str) -> Result<Vec<Task>, String> {
    let output_path = path.replace(INPUT_ENDS_CSV, OUTPUT_ENDS_CSV);
    let highlights_path = path.replace(INPUT_ENDS_CSV, HIGHLIGHT_ENDS_CSV);

    if !std::path::Path::new(output_path.as_str()).exists() {
        fs::write(output_path.as_str(), "").map_err(|e| e.to_string())?;
    }

    if !std::path::Path::new(highlights_path.as_str()).exists() {
        fs::write(highlights_path.as_str(), "").map_err(|e| e.to_string())?;
    }

    let file_inputs = read_csv_inputs(path)?;
    let file_outputs = read_csv_outputs(output_path.as_str())?;
    let file_highlights = read_csv_highlights(highlights_path.as_str())?;
    Ok(file_inputs
        .into_iter()
        .map(|input| {
            let output = file_outputs.iter().find(|o| o.text_id == input.id).cloned();
            let highlights = file_highlights
                .iter()
                .filter(|h| h.text_id == input.id)
                .cloned()
                .collect();
            Task {
                input,
                output,
                highlights,
                origin: path.to_string(),
                postponed: None,
            }
        })
        .collect())
}

#[tauri::command]
pub fn read_json_file_tasks(path: &str) -> Result<Vec<Task>, String> {
    let output_path = path.replace(INPUT_ENDS_JSON, OUTPUT_ENDS_CSV);
    let highlights_path = path.replace(INPUT_ENDS_JSON, HIGHLIGHT_ENDS_CSV);

    if !std::path::Path::new(output_path.as_str()).exists() {
        fs::write(output_path.as_str(), "").map_err(|e| e.to_string())?;
    }

    if !std::path::Path::new(highlights_path.as_str()).exists() {
        fs::write(highlights_path.as_str(), "").map_err(|e| e.to_string())?;
    }

    let file_inputs = read_json_inputs(path)?;
    let file_outputs = read_csv_outputs(output_path.as_str())?;
    let file_highlights = read_csv_highlights(highlights_path.as_str())?;
    Ok(file_inputs
        .into_iter()
        .map(|input| {
            let output = file_outputs.iter().find(|o| o.text_id == input.id).cloned();
            let highlights = file_highlights
                .iter()
                .filter(|h| h.text_id == input.id)
                .cloned()
                .collect();
            Task {
                input,
                output,
                highlights,
                origin: path.to_string(),
                postponed: None,
            }
        })
        .collect())
}

#[tauri::command]
pub fn write_task(task: Task) -> Result<(), String> {
    let is_json_origin = task.origin.ends_with(INPUT_ENDS_JSON);

    let (highlights_path, output_path) = if is_json_origin {
        (
            task.origin.replace(INPUT_ENDS_JSON, HIGHLIGHT_ENDS_CSV),
            task.origin.replace(INPUT_ENDS_CSV, OUTPUT_ENDS_CSV),
        )
    } else {
        (
            task.origin.replace(INPUT_ENDS_CSV, HIGHLIGHT_ENDS_CSV),
            task.origin.replace(INPUT_ENDS_CSV, OUTPUT_ENDS_CSV),
        )
    };

    write_csv_highlights(&highlights_path, task.highlights)?;

    if let Some(output) = task.output {
        write_csv_output(&output_path, output)?;
    }

    if is_json_origin {
        
    }

    Ok(())
}

#[tauri::command]
pub fn read_dir_tasks(dir: &str) -> Result<Vec<Task>, String> {
    let dir = fs::read_dir(dir).map_err(|e| e.to_string())?;

    let mut tasks = vec![];

    for entry in dir {
        let entry = entry.map_err(|e| e.to_string())?;
        if let Some(name) = entry.file_name().to_str() {
            if name.ends_with(INPUT_ENDS_CSV) {
                if let Some(path) = entry.path().to_str() {
                    tasks.extend(read_csv_file_tasks(path)?)
                }
            } else if name.ends_with(INPUT_ENDS_JSON) {
                if let Some(path) = entry.path().to_str() {
                    tasks.extend(read_json_file_tasks(path)?)
                }
            }
        }
    }

    Ok(tasks)
}
