use freya::prelude::*;

use crate::use_app_theme;

#[component]
pub fn Card(
    children: Element,
    on_click: Option<EventHandler<()>>,
    selected: Option<bool>,
) -> Element {
    let theme = use_app_theme();
    let mut hover = use_signal(|| false);
    let mut active = use_signal(|| false);

    rsx!(
        rect {
            padding: "10",
            width: "fill",
            onpointerenter: move |_| { hover.set(true) },
            onpointerleave: move |_| { hover.set(false) },
            onpointerdown: move |_| { active.set(true) },
            onpointerup: move |_| { active.set(false) },
            onclick: move |_| {
                if let Some(handler) = on_click {
                    handler.call(());
                }
            },
            rect {
                background: theme().surface(1).to_string(),
                padding: "10",
                width: "100%",
                corner_radius: "8",
                border_align: "inner",
                border: {
                    let color = theme().accent();
                    if hover() || selected == Some(true) {
                        format!("2 solid {color}")
                    } else {
                        format!("2 solid {}", color.greyscale())
                    }
                },
                shadow: if hover() { Some(theme().shadow(1)) } else { None },
                {children}
            }
        }
    )
}
