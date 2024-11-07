#![allow(non_snake_case)]
#![allow(unused_imports)]

mod layout;
mod pages;
mod route;
mod brower;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use dioxus::*;

use crate::route::Route;

/* const STYLE: Asset = asset!("/examples/assets/router.css");

fn main()
{
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element
{
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}
 */

const STYLE: Asset = asset!("/assets/router.css");

fn main()
{
    dioxus::launch(|| {
        rsx! {
            document::Link { rel: "stylesheet", href: STYLE }
            Router::<Route> {}
        }
    });
}
