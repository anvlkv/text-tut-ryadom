use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ProjectEntryView {
    pub id: String,
    pub at: usize,
    pub text: Vec<String>,
    pub highlights: Vec<Highlight>,
    pub summary: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Highlight {
    pub start: usize,
    pub end: usize,
    pub rubric: Uuid,
}
