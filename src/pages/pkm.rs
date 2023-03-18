use crate::Pokemon;
use crate::{components::BackButton, get_pokemon, set_complete, set_incomplete};
use dioxus::prelude::*;
use rusqlite::Connection;
use std::ops::Deref;

#[inline_props]
pub fn Pkm(cx: Scope) -> Element {
    let conn = use_shared_state::<Connection>(cx).unwrap();
    let id = dioxus_router::use_route(cx)
        .last_segment()
        .unwrap()
        .parse::<usize>()
        .unwrap_or_default();
    let path = dioxus_router::use_route(cx)
        .query_param("path")
        .map(|s| s.to_string())
        .unwrap_or("/".to_string());

    let pokemon = use_ref(cx, || get_pokemon(id, conn.read().deref()));

    let url = format!(
        "https://smogon.com/dex/media/sprites/xy/{}.gif",
        pokemon
            .read()
            .pokemon
            .replace(' ', "-")
            .replace('.', "")
            .replace('\'', "")
            .to_lowercase()
    );

    let color = if pokemon.read().complete {
        "bg-green-300"
    } else {
        "bg-red-300"
    };

    cx.render(rsx! {
        div {
            class: "min-h-screen flex flex-col flex-wrap justify-center items-center text-center",
            BackButton {path: "{path}"},
            div {
                class: "rounded-lg hover:cursor-pointer",
                style: "height: 130px; width: 130px; background-repeat: no-repeat; background-size: contain; background-position: center center; background-image: url(\"{url}\");",
                ""
            },
            pokemon.with(|p| {
                let Pokemon {
                    main_format,
                    set_format,
                    set_name,
                    pokemon,
                    item,
                    ability,
                    evs,
                    ivs,
                    moves,
                    nature,
                    ..
                } = p.clone();
                rsx! {
                    p {
                        class: "my-4",
                        "Main Format: {main_format}"
                        br {}
                        "Set Format: {set_format}"
                        br {}
                        "Set Name: {set_name}"
                        br {}
                        "{pokemon} @ {item}"
                        br {}
                        "Ability: {ability}"
                        br {}
                        "EVs: {evs}",
                        if !ivs.is_empty() {
                            rsx!{
                                br {}
                                "IVs: {ivs}"
                            }
                        },
                        br {}
                        "{nature} Nature"
                        moves.split('\n').map(|m| {
                            let m = m.to_string();
                            rsx! {
                                br {}
                                "{m}"
                            }
                        })
                    }
                }
            }),
            button {
                class: "py-2 px-4 {color} rounded-md",
                onclick: move |_| {
                    pokemon.with_mut(|pkm| if pkm.complete { set_incomplete(pkm.id, conn.read().deref()); pkm.complete = false; } else { set_complete(pkm.id, conn.read().deref()); pkm.complete = true; });
                },
                if pokemon.read().complete {
                    "Undo Complete"
                } else {
                    "Complete"
                }
            }
        },
    })
}
