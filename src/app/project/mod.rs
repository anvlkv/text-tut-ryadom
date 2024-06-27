mod select;
mod summarize;

pub use select::*;
pub use summarize::*;

use dioxus_router::prelude::Outlet;
use freya::prelude::*;

use super::routes::AppRoute;

#[component]
pub fn Project() -> Element {
    rsx!(Body {
        label {
            "Project"
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
