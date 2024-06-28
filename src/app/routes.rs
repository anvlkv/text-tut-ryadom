#![allow(non_snake_case)]

use dioxus_router::prelude::{Routable, Router};
use freya::prelude::*;

use super::{guide::*, home::*, project::*, setup::*};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum AppRoute {
    #[route("/")]
    Home,
    #[nest("/setup")]
        #[layout(Setup)]
            #[route("/text")]
            TextSetup {},
            #[route("/theme")]
            ThemeSetup { },
            #[nest("/data")]
                #[route("/")]
                DataSetup { },
                #[route("/label-studio")]
                LabelStudioSrcSetup { },
                #[route("/local")]
                LocalSrcSetup { },
            #[end_nest]
        #[end_layout]
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

// ANCHOR: router
// #[derive(Routable, Clone)]
// #[rustfmt::skip]
// enum Route {
//     #[layout(NavBar)]
//         #[route("/")]
//         Home {},
//         #[nest("/blog")]
//             #[layout(Blog)]
//                 #[route("/")]
//                 BlogList {},
//                 #[route("/post/:name")]
//                 BlogPost { name: String },
//             #[end_layout]
//         #[end_nest]
//     #[end_layout]
//     #[nest("/myblog")]
//         #[redirect("/", || Route::BlogList {})]
//         #[redirect("/:name", |name: String| Route::BlogPost { name })]
//     #[end_nest]
//     #[route("/:..route")]
//     PageNotFound {
//         route: Vec<String>,
//     },
// }
// ANCHOR_END: router

pub fn AppRouter() -> Element {
    rsx! { Router::<AppRoute> {} }
}

// #[allow(non_snake_case)]
// fn AppSidebar() -> Element {
//     rsx!(
//         NativeRouter {
//             Sidebar {
//                 sidebar: rsx!(
//                     Link {
//                         to: Route::Home,
//                         ActivableRoute {
//                             route: Route::Home,
//                             // exact: true,
//                             SidebarItem {
//                                 label {
//                                     "Go to Hey ! 👋"
//                                 }
//                             },
//                         }
//                     },
//                     Link {
//                         to: Route::Wow,
//                         ActivableRoute {
//                             route: Route::Wow,
//                             SidebarItem {
//                                 label {
//                                     "Go to Wow! 👈"
//                                 }
//                             },
//                         }
//                     },
//                     SidebarItem {
//                         onclick: |_| println!("Hello!"),
//                         label {
//                             "Print Hello! 👀"
//                         }
//                     },
//                 ),
//                 Body {
//                     rect {
//                         main_align: "center",
//                         cross_align: "center",
//                         width: "100%",
//                         height: "100%",
//                         Outlet::<Route> {  }
//                     }
//                 }
//             }
//         }
//     )
// }

#[allow(non_snake_case)]
#[component]
fn PageNotFound() -> Element {
    rsx!(
        label {
            "404!! 😵"
        }
    )
}
