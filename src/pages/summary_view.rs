use crate::components::BackButton;
use crate::{get_pokemon_list, Format};
use dioxus::prelude::*;
use dioxus_router::Link;
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use rusqlite::Connection;
use std::ops::Deref;

#[inline_props]
pub fn SummaryView(cx: Scope, format: Format) -> Element {
    let conn = use_shared_state::<Connection>(cx).unwrap();

    let pokemon = use_ref(cx, || get_pokemon_list(*format, conn.read().deref()));

    let prev_path = format!("/{}", format.to_string().to_lowercase());

    cx.render(rsx! {
        div {
            class: "min-h-screen",
            BackButton {path: "/"},
            div {
                class: "flex flex-col flex-wrap justify-center content-center py-4 items-center",
                button {
                    class: "mb-4 h-min w-min py-4 px-6 bg-blue-400",
                    onclick: move |_| {
                        let path = format!("/{}", format.to_string().to_lowercase());
                        let mut rng = thread_rng();
                        if let Some(&(id, _, _)) = pokemon.read().iter().filter(|(_, _, complete)| !complete).choose(&mut rng) {
                            dioxus_router::use_router(cx).navigate_to(&format!("/pkm/{id}?path={path}"));
                        }
                    },
                    "Randomize"
                }
                div {
                    class: "grid grid-cols-5 gap-6",
                    pokemon.read().iter().map(|(id, pokemon, complete)| {
                        let url = format!("https://smogon.com/dex/media/sprites/xy/{}.gif", pokemon.replace(' ', "-").replace('.', "").replace('\'', "").to_lowercase());
                        let path = prev_path.clone();
                        let color = if *complete {
                            "bg-green-300"
                        } else {
                            "bg-red-300"
                        };
                        rsx! {
                            div {
                                class: "rounded-md hover:cursor-pointer {color}",
                                style: "height: 130px; width: 130px; background-repeat: no-repeat; background-size: contain; background-position: center center; background-image: url(\"{url}\");",
                                Link {
                                    to: "/pkm/{id}?path={path}",
                                    class: "inline-block h-full w-full",
                                    ""
                                }
                            }
                        }
                    })
                }
            }
        },
    })
}
