use crate::route::Route;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[component]
pub fn Home() -> Element
{
    rsx! { h1 { "Welcome to the Dioxus Blog!" } }
}
