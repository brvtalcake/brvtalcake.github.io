use leptos::prelude::*;

#[macro_use]
extern crate turf;

mod fonts;

#[component]
fn Top() -> impl IntoView
{
	todo!();
	()
}

#[component]
fn Website() -> impl IntoView
{
	view! { <p>"Hello, world!"</p> }
}

fn main()
{
	console_error_panic_hook::set_once();
	leptos::mount::mount_to_body(Website)
}
