mod data;
mod guide;
mod home;
mod project;
mod routes;
mod setup;

use freya::prelude::*;

use crate::{
    resource::{get_stored_state, store_state},
    state::{AppState, StateCtx},
    use_app_theme, use_font,
};

pub fn app() -> Element {
    let restore_state = use_resource(move || get_stored_state());

    let mut state = use_signal(|| AppState::default());
    let store_state = use_callback(move || {
        let data = state.read().clone();
        spawn(async move {
            store_state(data).await;
        });
    });

    use_effect(move || match restore_state.read().as_ref() {
        Some(Some(d)) => {
            *state.write() = d.clone();
        }
        Some(None) => {
            log::info!("no state data");
        }
        _ => {
            log::error!("error reading state");
        }
    });

    provide_context(StateCtx(state, store_state));

    let font = use_font();
    let theme = state.map(|s| {
        s.config
            .as_ref()
            .map(|c| c.theme.as_ref())
            .flatten()
            .unwrap_or(&crate::util::Theme::Light)
    });

    rsx!(
        Body { 
            rect {
                width: "100%",
                height: "100%",
                font_family: font().font_family,
                font_size: font().font_size.to_string(),
                main_align: "center",
                cross_align: "center",
                background: theme().surface(0).to_string(),
                color: theme().text().to_string(),
                routes::AppRouter {}
            }
        }
    )
}
