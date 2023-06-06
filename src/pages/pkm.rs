use crate::Pokemon;
use crate::{components::BackButton, get_pokemon, set_complete, set_incomplete};
use dioxus::prelude::*;
use rusqlite::Connection;
use smog_strat_dex_rs::Generation;
use std::ops::Deref;

#[inline_props]
pub fn Pkm(cx: Scope) -> Element {
    let conn = use_shared_state::<Connection>(cx).unwrap();
    let id = dioxus_router::use_route(cx)
        .last_segment()
        .unwrap()
        .parse::<usize>()
        .unwrap_or_default();
    let format = dioxus_router::use_route(cx)
        .query_param("format")
        .map(|s| s.to_string())
        .unwrap_or("OU".to_string());

    let gen = dioxus_router::use_route(cx)
        .query_param("gen")
        .map(|s| Generation::try_from(s.as_ref()).unwrap_or(Generation::ScarletViolet))
        .unwrap_or(Generation::ScarletViolet);
    let gen_str: &'static str = gen.into();
    let pokemon = use_ref(cx, || get_pokemon(id, conn.read().deref(), gen));

    let url = format!(
        "https://smogon.com/dex/media/sprites/xy/{}.gif",
        pokemon
            .read()
            .pokemon
            .replace(' ', "-")
            .replace(['.', '\''], "")
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
            BackButton {path: "/summary?format={format}&gen={gen_str}"},
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
                    tera_type,
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
                        "{pokemon}",
                        if !item.is_empty() {
                            rsx! {
                                " @ {item}"
                            }
                        },
                        if !ability.is_empty() {
                            rsx! {
                                br {}
                                "Ability: {ability}"
                            }
                        }
                        if !tera_type.is_empty() {
                            rsx!{
                                br {}
                                "Tera Type: {tera_type}"
                            }
                        },
                        if !evs.is_empty() {
                            rsx! {
                                br {}
                                "EVs: {evs}"
                            }
                        },
                        if !ivs.is_empty() {
                            rsx!{
                                br {}
                                "IVs: {ivs}"
                            }
                        },
                        if !nature.is_empty() {
                            rsx! {
                                br {},
                                "{nature} Nature"
                            }
                        },
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
                    pokemon.with_mut(|pkm| if pkm.complete { set_incomplete(pkm.id, conn.read().deref(), gen); pkm.complete = false; } else { set_complete(pkm.id, conn.read().deref(), gen); pkm.complete = true; });
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
