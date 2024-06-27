// mod continue;
// mod setup;

use freya::prelude::*;
use rust_i18n::t;

use crate::{
    app::{routes::AppRoute, setup},
    state::StateCtx,
};

#[component]
pub fn Home() -> Element {
    let StateCtx(state, _) = use_context::<StateCtx>();

    rsx!(Body {
        if state().data_src.is_some() {
            // Link {
            //     to: Route::EntrySelect { entry: () },

            // }
            label {
                {t!("project")}
            }
        }
        Link {
            to: AppRoute::TextSetup {},
            label {
                {t!("configure")}
            }
        }
    })
}
