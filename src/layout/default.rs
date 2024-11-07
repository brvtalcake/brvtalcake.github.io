use crate::route::Route;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};


#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::BlogHome {}, "Blog" }
        }
        Outlet::<Route> {}
    }
}

#[component]
pub fn Blog() -> Element {
    rsx! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}