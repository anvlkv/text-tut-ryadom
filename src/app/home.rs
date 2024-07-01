use freya::prelude::*;
use rust_i18n::t;

use crate::{
    app::{routes::AppRoute, setup},
    components::Card,
    state::StateCtx,
    use_approach,
};

#[component]
pub fn Home() -> Element {
    let StateCtx(state, _) = use_context::<StateCtx>();
    let approach = use_approach();

    rsx!(
        rect {
            if state().config.is_some() {
                if state().data_src.is_some() {
                    // Link {
                    //     to: Route::EntrySelect { entry: () },

                    // }
                    label { {t!("project")} }
                    Link { to: AppRoute::AllDataSetup {}, Card { 
                        label { {t!(approach().key("data.reselect").as_str())} }
                    } }
                } else {
                    Link { to: AppRoute::AllDataSetup {}, Card { 
                        label { {t!(approach().key("data.setup").as_str())} }
                    } }
                }

                Link { to: AppRoute::ProjectGuide {}, Card { 
                    label { {t!(approach().key("home.guide").as_str())} }
                } }
            }
            Link { to: AppRoute::AllSetup {}, Card { 
                label { {t!(approach().key("home.configure").as_str())} }
            } }
        }
    )
}
