use freya::prelude::*;
use rust_i18n::t;

use crate::{
    state::{AppStateEvent, StateCtx},
    use_approach, use_font, MAX_SIZE, MIN_SIZE,
};

#[component]
pub fn TextSetup() -> Element {
    let StateCtx(mut state, _) = use_context();
    let approach = use_approach();
    let font = use_font();

    let dif = MAX_SIZE - MIN_SIZE;

    let text = t!("util.example_text").to_string();
    let lines = text.lines();

    rsx!(
        rect {
            width: "100%",
            main_align: "center",
            cross_align: "center",
            padding: "20",
            label { {t!(approach().key("setup.font_size_setup").as_str())} }
            rect {
                width: "100%",
                direction: "horizontal",
                main_align: "center",
                cross_align: "center",
                rect {
                    width: "calc(100% / 3)",
                    main_align: "center",
                    cross_align: "center",
                    label { font_size: MIN_SIZE.to_string(), "Aa" }
                }
                rect {
                    width: "calc(100% / 3)",
                    main_align: "center",
                    cross_align: "center",
                    label { font_size: (MIN_SIZE + dif / 2.0).to_string(), "Aa" }
                }
                rect {
                    width: "calc(100% / 3)",
                    main_align: "center",
                    cross_align: "center",
                    label { font_size: MAX_SIZE.to_string(), "Aa" }
                }
            }
            Slider {
                width: "80%",
                value: ((font().font_size - MIN_SIZE) / dif) * 100.0,
                onmoved: move |e| {
                    state.write().update(AppStateEvent::SetupFontSize((e / 100.0) * dif + MIN_SIZE))
                }
            }
            label { {t!("setup.font_size_setup.example")} }
            ScrollView { direction: "vertical",
                for line in lines {
                    paragraph { margin: "5",
                        text { {line} }
                    }
                }
            }
        }
    )
}
