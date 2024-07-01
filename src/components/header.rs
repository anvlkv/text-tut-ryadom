use freya::prelude::*;

use crate::use_app_theme;

#[component]
pub fn Header(children: Element, height: Signal<f32>) -> Element {
    let theme = use_app_theme();

    let (node_ref, layout) = use_node_signal();

    use_effect(move || height.set(layout().area.height()));

    rsx!(
        rect {
            width: "100%",
            padding: "10",
            background: theme().surface(2).to_string(),
            shadow: theme().shadow(2),
            main_align: "center",
            cross_align: "center",
            reference: node_ref,
            { children }
        }
    )
}
