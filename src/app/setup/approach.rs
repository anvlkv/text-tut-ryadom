use freya::prelude::*;
use rust_i18n::t;

use crate::{
    components::Card,
    state::{AppStateEvent, StateCtx},
    use_approach, use_font_with_options, Approach, FontOptionsBuilder, FontValue,
};

#[component]
pub fn ApproachSetup() -> Element {
    let StateCtx(mut state, _) = use_context();
    let approach = use_approach();
    let font = use_font_with_options(
        FontOptionsBuilder::default()
            .size(FontValue::MuchLesser)
            .build()
            .unwrap(),
    );

    rsx!(
        rect {
            width: "100%",
            main_align: "center",
            cross_align: "center",
            padding: "20",
            label { {t!(approach().key("setup.approach_setup").as_str())} }
            rect { direction: "horizontal",
                rect { width: "50%",
                    Card {
                        on_click: move |_| {
                            state.write().update(AppStateEvent::SetupApproach(Approach::Polite));
                        },
                        selected: state()
                            .config
                            .map(|c| c.approach.map(|a| a == Approach::Polite))
                            .flatten()
                            .unwrap_or_default(),
                        label { font_size: font().font_size.to_string(), {t!("setup.approach_setup.label_polite")} }
                        label { {t!("setup.approach_setup.option_polite")} }
                    }
                }
                rect { width: "50%",
                    Card {
                        on_click: move |_| {
                            state.write().update(AppStateEvent::SetupApproach(Approach::Familiar));
                        },
                        selected: state()
                            .config
                            .map(|c| c.approach.map(|a| a == Approach::Familiar))
                            .flatten()
                            .unwrap_or_default(),
                        label { font_size: font().font_size.to_string(), {t!("setup.approach_setup.label_familiar")} }
                        label { {t!("setup.approach_setup.option_familiar")} }
                    }
                }
            }
        }
    )
}
