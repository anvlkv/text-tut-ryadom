mod home;
mod project;
mod routes;
mod setup;

use freya::prelude::*;

use crate::{
    resource::{get_stored_state, store_state},
    state::{AppState, StateCtx},
    FIRA_SANS_REGULAR,
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

    rsx!(
        rect{
            font_family: FIRA_SANS_REGULAR.0,
            routes::AppRouter {}
        }
    )
}
