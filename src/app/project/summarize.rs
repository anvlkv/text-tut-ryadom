use freya::prelude::*;

#[component]
pub fn EntrySummarize(entry: String) -> Element {
    rsx!(Body {
        label {
            "EntrySummarize"
        }
    })
}
