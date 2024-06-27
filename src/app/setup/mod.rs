mod data;
mod label_studio_src;
mod local_src;
mod text;
mod theme;

pub use data::*;
pub use label_studio_src::*;
pub use local_src::*;
pub use text::*;
pub use theme::*;

use dioxus_router::prelude::Outlet;
use freya::prelude::*;

use super::routes::AppRoute;

#[component]
pub fn Setup() -> Element {
    rsx!(Body {
        label {
            "Setup"
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
