use freya::prelude::*;
use rust_i18n::t;
use strum::VariantArray;

use crate::{
    components::Card,
    state::{AppStateEvent, StateCtx},
    use_approach,
    util::Theme,
};

const IMG_LIGHT: &[u8] = include_bytes!("./theme/Light.png");
const IMG_DARK: &[u8] = include_bytes!("./theme/Dark.png");

#[component]
pub fn ThemeSetup() -> Element {
    let variants = Theme::VARIANTS;
    let approach = use_approach();
    let StateCtx(mut state, _) = use_context();

    rsx!(
        rect {
            width: "100%",
            main_align: "center",
            cross_align: "center",
            padding: "20",
            label { {t!(approach().key("setup.theme_setup").as_str())} }
            rect { direction: "horizontal",
                for th in variants.iter() {
                    rect { width: format!("calc(100% / {})", variants.len()),
                        Card {
                            on_click: move |_| {
                                state.write().update(AppStateEvent::SetupTheme(*th));
                            },
                            selected: state().config.map(|c| c.theme.map(|t| t == *th)).flatten().unwrap_or_default(),
                            rect {
                                image {
                                    width: "100",
                                    height: "100",
                                    image_data: static_bytes(
                                        match th {
                                            Theme::Dark => IMG_DARK,
                                            Theme::Light => IMG_LIGHT,
                                        },
                                    )
                                }
                                label {
                                    {t!(format!("setup.theme_setup.theme.{}", th.to_string().to_lowercase()).as_str())}
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
