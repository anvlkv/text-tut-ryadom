use freya::prelude::*;
use rust_i18n::t;

use crate::{
    app::routes::AppRoute,
    components::Footer,
    resource::DataSrc,
    state::{AppStateEvent, StateCtx},
    use_app_theme, use_approach,
};

#[component]
pub fn LocalSrcSetup() -> Element {
    let approach = use_approach();
    let theme = use_app_theme();
    let StateCtx(mut state, save_state) = use_context();
    let mut data = use_context::<Signal<Option<Result<DataSrc, std::io::Error>>>>();

    let open_dir = use_callback(move || {
        spawn(async move {
            if let Some(dir) = rfd::AsyncFileDialog::new().pick_folder().await {
                let path = dir.path();
                let read_data = DataSrc::open_local(path.into()).await;
                data.set(Some(read_data));
            } else {
                data.set(None);
            }
        });
    });

    use_effect(move || open_dir.call());

    rsx!(
        rect {
            Button { onclick: move |_| open_dir.call(),
                label { {t!(approach().key("data.reselect").as_str())} }
            }
            match &*data.read_unchecked() {
                Some(Ok(d)) => {
                    rsx!(
                        label{
                            {format!("data: {d:?}")}
                        }
                    )
                },
                Some(Err(e)) => {
                    rsx!(label {
                        {e.to_string()}
                    })
                },
                None => {
                    rsx!(label {
                        {"nthting selected"}
                    })
                }
            }
        }
    )
}
