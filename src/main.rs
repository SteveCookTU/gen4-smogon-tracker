#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dioxus::core::Element;
use dioxus::prelude::*;
use dioxus_desktop::tao::window::Icon;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_router::{Route, Router};
use gen4_smogon_tracker::pages::{home::Home, pkm::Pkm, summary_view::SummaryView};
use gen4_smogon_tracker::{initialize_db, Format};

static ICON: &[u8] = include_bytes!("../icon.ico");

#[tokio::main]
async fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new().with_window(
            WindowBuilder::new()
                .with_title("Gen4 Smogon Tracker")
                .with_window_icon(Some(load_icon()))
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(1280.0, 720.0)),
        ),
    );
}

fn load_icon() -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        // alternatively, you can embed the icon in the binary through `include_bytes!` macro and use `image::load_from_memory`
        let image = image::load_from_memory(ICON)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || initialize_db());

    cx.render(rsx! {
        style { include_str!("../output.css") }
        div {
            class: "bg-slate-400",
                Router {
                Route { to: "/", Home {} }
                Route { to: "/ou", SummaryView { format: Format::OU } }
                Route { to: "/uber", SummaryView { format: Format::Uber } }
                Route { to: "/uu", SummaryView { format: Format::UU } }
                Route { to: "/nu", SummaryView { format: Format::NU } }
                Route { to: "/nubl", SummaryView { format: Format::NUBL } }
                Route { to: "/uubl", SummaryView { format: Format::UUBL } }
                Route { to: "/lc", SummaryView { format: Format::LC } }
                Route { to: "/nfe", SummaryView { format: Format::NFE } }
                Route { to: "/pkm/:id", Pkm { } }
            }
        }
    })
}
