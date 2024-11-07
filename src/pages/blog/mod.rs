use crate::route::Route;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[component]
pub fn BlogHome() -> Element {
    rsx! {
        h2 { "Choose a post" }
        div { id: "blog-list",
            Link { to: Route::BlogPost { name: "Blog post 1".into() },
                "Read the first blog post"
            }
            Link { to: Route::BlogPost { name: "Blog post 2".into() },
                "Read the second blog post"
            }
        }
    }
}

// We can use the `name` slug to show a specific blog post
// In theory we could read from the filesystem or a database here
#[component]
pub fn BlogPost(name: String) -> Element {
    let contents = match name.as_str() {
        "Blog post 1" => "This is the first blog post. It's not very interesting.",
        "Blog post 2" => "This is the second blog post. It's not very interesting either.",
        _ => "This blog post doesn't exist.",
    };

    rsx! {
        h2 { "{name}" }
        p { "{contents}" }
    }
}