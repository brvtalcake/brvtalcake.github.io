use leptos::prelude::*;

mod fonts;

#[component]
fn Top() -> impl IntoView
{
	let fonts = fonts::REGISTERED_FONTS.read().unwrap();
	let views = fonts
		.iter()
		.map(|el| {
			let class_name = el.to_class_name();
			let full_name = el.full_name();
			view! { <p class=class_name>Test of rendering some text with the {full_name.clone()} font</p> }
		})
		.collect_view();
	view! {
		<p>TODO !</p>
		{views}
	}
}

#[component]
fn Website() -> impl IntoView
{
	view! {
		<Top />
		<p>"Hello, world!"</p>
	}
}

fn main()
{
	console_error_panic_hook::set_once();
	leptos::mount::mount_to_body(Website)
}
