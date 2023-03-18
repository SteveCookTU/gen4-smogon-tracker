use dioxus::prelude::*;
use dioxus_router::Link;

#[inline_props]
pub fn BackButton<'a>(cx: Scope, path: &'a str) -> Element {
    cx.render(rsx! {
        Link {
            to: "{path}",
            class: "absolute py-4 px-6 left-4 top-4 text-center bg-red-300 rounded-lg shadow-md transition duration-300 hover:cursor-pointer hover:bg-red-100",
            "Back"
        }
    })
}
