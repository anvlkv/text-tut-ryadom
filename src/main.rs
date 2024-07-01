#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate derive_builder;

mod app;
mod components;
mod resource;
mod state;
mod util;

use freya::prelude::*;
use log::LevelFilter;
use rust_i18n::*;
use util::*;

i18n!();

fn main() {
    let level = LevelFilter::Trace;
    if let Some(logs_path) = resource::logs_path() {
        println!("logging to: {logs_path:?}");
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
            .with_font(FIRA_SANS_THIN.0, FIRA_SANS_THIN.1)
            .with_font(FIRA_SANS_THIN_ITALIC.0, FIRA_SANS_THIN_ITALIC.1)
            .with_font(FIRA_SANS_EXTRA_LIGHT.0, FIRA_SANS_EXTRA_LIGHT.1)
            .with_font(
                FIRA_SANS_EXTRA_LIGHT_ITALIC.0,
                FIRA_SANS_EXTRA_LIGHT_ITALIC.1,
            )
            .with_font(FIRA_SANS_LIGHT.0, FIRA_SANS_LIGHT.1)
            .with_font(FIRA_SANS_LIGHT_ITALIC.0, FIRA_SANS_LIGHT_ITALIC.1)
            .with_font(FIRA_SANS_REGULAR.0, FIRA_SANS_REGULAR.1)
            .with_font(FIRA_SANS_ITALIC.0, FIRA_SANS_ITALIC.1)
            .with_font(FIRA_SANS_MEDIUM.0, FIRA_SANS_MEDIUM.1)
            .with_font(FIRA_SANS_MEDIUM_ITALIC.0, FIRA_SANS_MEDIUM_ITALIC.1)
            .with_font(FIRA_SANS_SEMI_BOLD.0, FIRA_SANS_SEMI_BOLD.1)
            .with_font(FIRA_SANS_SEMI_BOLD_ITALIC.0, FIRA_SANS_SEMI_BOLD_ITALIC.1)
            .with_font(FIRA_SANS_BOLD.0, FIRA_SANS_BOLD.1)
            .with_font(FIRA_SANS_BOLD_ITALIC.0, FIRA_SANS_BOLD_ITALIC.1)
            .with_font(FIRA_SANS_EXTRA_BOLD.0, FIRA_SANS_EXTRA_BOLD.1)
            .with_font(FIRA_SANS_EXTRA_BOLD_ITALIC.0, FIRA_SANS_EXTRA_BOLD_ITALIC.1)
            .with_font(FIRA_SANS_BLACK.0, FIRA_SANS_BLACK.1)
            .with_font(FIRA_SANS_BLACK_ITALIC.0, FIRA_SANS_BLACK_ITALIC.1),
    );
}
