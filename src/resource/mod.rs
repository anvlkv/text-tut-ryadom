mod data_src;
mod state;

const ROOT: &str = "text-tut-ryadom";
const LOGS: &str = "app.log";

use std::path::PathBuf;

pub use data_src::*;
pub use state::*;

pub fn logs_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|d| {
        let mut path = d;
        path.push(ROOT);
        path.push(LOGS);
        path
    })
}
