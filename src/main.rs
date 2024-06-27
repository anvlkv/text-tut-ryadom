#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod resource;
mod state;

use freya::{hotreload::FreyaCtx, prelude::*};
use log::LevelFilter;
use rust_i18n::*;

i18n!();

pub static FIRA_SANS_REGULAR: (&str, &[u8]) = (
    "FiraSans-Regular",
    include_bytes!("../assets/Fira_Sans/FiraSans-Regular.ttf"),
);

fn main() {
    dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());

    let level = LevelFilter::Trace;
    if let Some(logs_path) = resource::logs_path() {
        _ = simple_logging::log_to_file(logs_path, level);
    } else {
        simple_logging::log_to_stderr(level)
    }

    set_locale("ru");

    let title = t!("title").to_string();

    launch_cfg(
        app::app,
        LaunchConfig::<()>::new()
            .with_window_attributes(move |a| a.with_title(title.clone()))
            .with_font(FIRA_SANS_REGULAR.0, FIRA_SANS_REGULAR.1),
    );
}
