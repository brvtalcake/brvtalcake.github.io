use crate::route::Route;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element
{
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
