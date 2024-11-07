use crate::pages::blog::{BlogHome, BlogPost};
use crate::pages::home::Home;
use crate::pages::not_found::PageNotFound;

use crate::layout;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Wrap Home in a Navbar Layout
    #[layout(layout::default::NavBar)]
        // The default route is always "/" unless otherwise specified
        #[route("/")]
        Home {},

        // Wrap the next routes in a layout and a nest
        #[nest("/blog")]
        #[layout(layout::default::Blog)]
            // At "/blog", we want to show a list of blog posts
            #[route("/")]
            BlogHome {},

            // At "/blog/:name", we want to show a specific blog post, using the name slug
            #[route("/:name")]
            BlogPost { name: String },

        // We need to end the blog layout and nest
        // Note we don't need either - we could've just done `/blog/` and `/blog/:name` without nesting,
        // but it's a bit cleaner this way
        #[end_layout]
        #[end_nest]

    // And the regular page layout
    #[end_layout]

    // Add some redirects for the `/myblog` route
    #[nest("/myblog")]
        #[redirect("/", || Route::BlogHome {})]
        #[redirect("/:name", |name: String| Route::BlogPost { name })]
    #[end_nest]

    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}