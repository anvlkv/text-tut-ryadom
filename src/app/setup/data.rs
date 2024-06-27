use dioxus_router::prelude::Outlet;
use freya::prelude::*;

use crate::app::routes::AppRoute;

#[component]
pub fn DataSetup() -> Element {
    rsx!(Body {
        label {
            "Data"
        }
        rect {
            main_align: "center",
            cross_align: "center",
            width: "100%",
            height: "90%",
            Outlet::<AppRoute> {  }
        }
    })
}
