mod entry;

pub use entry::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ProjectView {
    pub total_entries: usize,
    pub current_entry: ProjectEntryView,
}
