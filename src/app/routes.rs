#![allow(non_snake_case)]

use dioxus_router::prelude::{Routable, Router};
use freya::prelude::*;

use super::{data::*, guide::*, home::*, project::*, setup::*};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum AppRoute {
    #[route("/")]
    Home,
    #[nest("/setup")]
        #[layout(Setup)]
            #[route("/")]
            AllSetup {},
            #[route("/approach")]
            ApproachSetup {},
            #[route("/text")]
            TextSetup {},
            #[route("/theme")]
            ThemeSetup { },
        #[end_layout]
        #[nest("/data")]
            #[layout(DataSetup)]
                #[route("/")]
                AllDataSetup { },
                #[route("/label-studio")]
                LabelStudioSrcSetup { },
                #[route("/local")]
                LocalSrcSetup { },
            #[end_layout]
        #[end_nest]
    #[end_nest]
    #[nest("/guide")]
        #[layout(Guide)]
            #[route("/project")]
            ProjectGuide { },
            #[route("/select")]
            EntrySelectGuide { },
            #[route("/summarize")]
            EntrySummarizeGuide { },
        #[end_layout]
    #[end_nest]
    #[nest("/project")]
        #[layout(Project)]
            #[route("/:entry/select")]
            EntrySelect { entry: String },
            #[route("/:entry/summarize")]
            EntrySummarize { entry: String },
        #[end_layout]
    #[end_nest]
    #[route("/..route")]
    PageNotFound { },
}

pub fn AppRouter() -> Element {
    rsx! {
        Router::<AppRoute> {}
    }
}

#[allow(non_snake_case)]
#[component]
fn PageNotFound() -> Element {
    rsx!(
        label { "404!! 😵" }
    )
}
