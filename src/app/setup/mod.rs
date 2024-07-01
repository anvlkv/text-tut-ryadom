mod approach;
mod text;
mod theme;

pub use approach::*;
pub use text::*;
pub use theme::*;

use dioxus_router::{
    hooks::{use_navigator, use_route},
    prelude::Outlet,
};
use freya::prelude::*;
use rust_i18n::t;

use crate::{
    components::{Card, Footer, Header},
    state::StateCtx,
    use_app_theme, use_approach,
};

use super::routes::AppRoute;

#[component]
pub fn Setup() -> Element {
    let approach = use_approach();
    let theme = use_app_theme();
    let route = use_route();
    let nav = use_navigator();
    let steps = vec![
        AppRoute::AllSetup {},
        AppRoute::ApproachSetup {},
        AppRoute::TextSetup {},
        AppRoute::ThemeSetup {},
    ];
    let StateCtx(_, save_state) = use_context();

    let header_height = use_signal(|| 0_f32);
    let footer_height = use_signal(|| 0_f32);

    rsx!(
        rect {
            Header { height: header_height,
                label { {t!(approach().key("setup.configure").as_str())} }
            }
            rect {
                main_align: "center",
                cross_align: "center",
                width: "100%",
                height: format!("calc(100% - {} - {})", header_height(), footer_height()),
                Outlet::<AppRoute> {}
            }
            Footer { height: footer_height,
                Link { to: AppRoute::Home, Button { 
                    label { {t!(approach().key("util.cancel").as_str())} }
                } }
                Button {
                    onclick: move |_| {
                        save_state.call();
                        let step_at = steps.iter().position(|s| s == &route);
                        let next = step_at
                            .map(|s| steps.iter().nth(s + 1).cloned())
                            .flatten()
                            .unwrap_or(AppRoute::Home);
                        _ = nav.push(next);
                    },
                    theme: theme_with!(ButtonTheme { background : theme().accent().to_string().into() }),
                    label { {t!(approach().key("util.next").as_str())} }
                }
            }
        }
    )
}

#[component]
pub fn AllSetup() -> Element {
    let approach = use_approach();

    rsx!(
        rect {
            Link { to: AppRoute::ApproachSetup {}, Card { 
                label { {t!(approach().key("setup.approach_setup").as_str())} }
            } }
            Link { to: AppRoute::TextSetup {}, Card { 
                label { {t!(approach().key("setup.font_size_setup").as_str())} }
            } }
            Link { to: AppRoute::ThemeSetup {}, Card { 
                label { {t!(approach().key("setup.theme_setup").as_str())} }
            } }
        }
    )
}
