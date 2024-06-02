use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::task::Task;

pub const HIGHLIGHT_ENDS: &str = ".highlights.csv";

#[derive(Clone, Serialize, Deserialize, TS)]
pub struct HighlightRecord {
    pub text_id: String,
    pub group_id: String,
    pub start: usize,
    pub end: usize,
    pub color: String,
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

#[tauri::command]
pub fn write_highlights(file: &str, highlights: Vec<HighlightRecord>) -> Result<(), String> {
    let mut entries = read_highlights(file)?;
    let ids: HashSet<&str> = HashSet::from_iter(highlights.iter().map(|h| h.text_id.as_str()));
    entries.retain(|e| !ids.contains(e.text_id.as_str()));
    entries.extend(highlights);

    let mut writer = csv::Writer::from_path(file).map_err(|e| e.to_string())?;
    
    for record in entries {
        writer.serialize(record).map_err(|e| e.to_string())?;
    }

    writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}


#[tauri::command]
pub fn split_highlight_ranges(highlight: (usize, usize), task: Task) -> Vec<(usize, usize)> {
    let mut sel_rng = (highlight.0..highlight.1).into_iter().peekable();
    let mut inner =
        task.highlights
            .iter()
            .map(|h| (h.start..h.end))
            .fold(vec![], |mut acc, rng| {
                let rng_before = (&mut sel_rng)
                    .take_while(|i| !rng.contains(i))
                    .collect::<Vec<_>>();
                if !rng_before.is_empty() {
                    let start = *rng_before.first().unwrap();
                    let end = *rng_before.last().unwrap();
                    if start != end {
                        acc.push((start, end))
                    }
                }
                _ = (&mut sel_rng)
                    .take_while(|i| rng.contains(i))
                    .collect::<Vec<_>>();
                acc
            });

    if let Some(start) = sel_rng.next() {
        let last = if let Some(end) = sel_rng.last() {
            (start, end)
        } else if let Some(chunk) = inner.last() {
            (chunk.1, start)
        } else {
            (0, 0)
        };

        if last.0 != last.1 {
            inner.push(last)
        }
    }
    inner
}
