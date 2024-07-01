mod label_studio_src;
mod local_src;

pub use label_studio_src::*;
pub use local_src::*;

use dioxus_router::{
    hooks::{use_navigator, use_route},
    prelude::Outlet,
};
use freya::prelude::*;
use rust_i18n::t;

use crate::{
    app::routes::AppRoute,
    components::{Card, Footer, Header},
    resource::DataSrc,
    state::{AppStateEvent, StateCtx},
    use_app_theme, use_approach,
};

#[component]
pub fn DataSetup() -> Element {
    let approach = use_approach();
    let theme = use_app_theme();
    let route = use_route::<AppRoute>();
    let nav = use_navigator();

    let StateCtx(mut state, save_state) = use_context();
    let data = use_signal::<Option<Result<DataSrc, std::io::Error>>>(|| None);

    let header_height = use_signal(|| 0_f32);
    let footer_height = use_signal(|| 0_f32);

    provide_context(data);

    rsx!(
        rect {
            Header { height: header_height,
                label { {t!(approach().key("data.setup").as_str())} }
            }
            rect {
                main_align: "center",
                cross_align: "center",
                width: "100%",
                height: format!("calc(100% - {} - {})", header_height(), footer_height()),
                Outlet::<AppRoute> {}
            }
            if (route != AppRoute::AllDataSetup {}) {
                Footer { height: footer_height,
                    Link { to: AppRoute::Home, Button { 
                        label { {t!(approach().key("util.cancel").as_str())} }
                    } }
                    Button {
                        onclick: move |_| {
                            let d = &*data.read_unchecked();
                            if let Some(Ok(d)) = d.as_ref() {
                                state.write().update(AppStateEvent::SetDataSource(d.clone()));
                                save_state.call();
                                _ = nav.push(AppRoute::Home);
                            }
                        },
                        theme: theme_with!(ButtonTheme { background : theme().accent().to_string().into() }),
                        label { {t!(approach().key("util.done").as_str())} }
                    }
                }
            }
        }
    )
}

#[component]
pub fn AllDataSetup() -> Element {
    let approach = use_approach();

    rsx!(
        rect {
            Link { to: AppRoute::LocalSrcSetup {}, Card { 
                label { {t!(approach().key("data.local").as_str())} }
            } }
            Link { to: AppRoute::LabelStudioSrcSetup {}, Card { 
                label { {t!(approach().key("data.label_studio").as_str())} }
            } }
        }
    )
}
