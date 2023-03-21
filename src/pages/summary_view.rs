use crate::components::BackButton;
use crate::{get_pokemon_list, Format};
use dioxus::prelude::*;
use dioxus_router::{use_route, Link};
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use rusqlite::Connection;
use smog_strat_dex_rs::Generation;
use std::ops::Deref;

#[inline_props]
pub fn SummaryView(cx: Scope) -> Element {
    let conn = use_shared_state::<Connection>(cx).unwrap();
    let format = use_route(cx)
        .query_param("format")
        .map(|s| Format::from(s.as_ref()))
        .unwrap_or(Format::OU);
    let gen = use_route(cx)
        .query_param("gen")
        .map(|s| Generation::try_from(s.as_ref()).unwrap_or(Generation::ScarletViolet))
        .unwrap_or(Generation::ScarletViolet);
    let pokemon = use_ref(cx, || get_pokemon_list(format, conn.read().deref(), gen));

    let gen_str: &'static str = gen.into();
    let format_str = format.to_string();

    cx.render(rsx! {
        div {
            class: "min-h-screen",
            BackButton {path: "/formats?gen={gen_str}"},
            div {
                class: "flex flex-col flex-wrap justify-center content-center py-4 items-center",
                button {
                    class: "mb-4 h-min w-min py-4 px-6 bg-blue-400",
                    onclick: move |_| {
                        let path = format!("/{}", format.to_string().to_lowercase());
                        let mut rng = thread_rng();
                        if let Some(&(id, _, _)) = pokemon.read().iter().filter(|(_, _, complete)| !complete).choose(&mut rng) {
                            dioxus_router::use_router(cx).navigate_to(&format!("/pkm/{id}?path={path}&gen={gen_str}"));
                        }
                    },
                    "Randomize"
                }
                div {
                    class: "grid grid-cols-5 gap-6",
                    pokemon.read().iter().map(|(id, pokemon, complete)| {
                        let url = format!("https://smogon.com/dex/media/sprites/xy/{}.gif", pokemon.replace(' ', "-").replace('.', "").replace('\'', "").to_lowercase());
                        let form = format_str.clone();
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
                                    to: "/pkm/{id}?format={form}&gen={gen_str}",
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
