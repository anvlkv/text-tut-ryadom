// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};

mod highlight;
mod input;
mod output;
mod task;

#[tauri::command]
async fn close_splashscreen(window: Window) {
    // Close splashscreen
    window
        .get_webview_window("splashscreen")
        .and_then(|w| w.close().ok())
        .unwrap_or_default();
    // Show main window
    window
        .get_webview_window("main")
        .expect("no window labeled 'main' found")
        .show()
        .unwrap();
}

fn main() {
    #[cfg(debug_assertions)]
    {
        <task::Task as ts_rs::TS>::export_all_to("../src/lib/types/").expect("write ts types");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            input::read_inputs,
            highlight::read_highlights,
            highlight::write_highlights,
            highlight::split_highlight_ranges,
            output::read_outputs,
            output::write_output,
            task::read_dir_tasks,
            task::read_file_tasks,
            task::write_task,
            close_splashscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
