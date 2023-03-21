use crate::components::BackButton;
use crate::{get_formats, initialize_db, initialize_db_data, table_exists, InitializationMessage};
use dioxus::prelude::*;
use dioxus_router::{use_route, Link};
use futures::StreamExt;
use smog_strat_dex_rs::Generation;

#[inline_props]
pub fn Formats(cx: Scope) -> Element {
    let gen = use_route(cx)
        .query_param("gen")
        .map(|s| Generation::try_from(s.as_ref()).unwrap_or(Generation::ScarletViolet))
        .unwrap_or(Generation::ScarletViolet);

    let loaded = use_state(cx, || table_exists(&initialize_db(), gen));
    let formats = use_ref(cx, || {
        if *loaded.get() == true {
            get_formats(&initialize_db(), gen)
        } else {
            Vec::new()
        }
    });
    let total = use_state(cx, || 0);
    let progress = use_state(cx, || 0);
    let initializing = use_state(cx, || false);
    let gen_str: &'static str = gen.into();

    let test = use_coroutine(cx, |mut rx| {
        let total = total.to_owned();
        let progress = progress.to_owned();
        let loaded = loaded.to_owned();
        let formats = formats.to_owned();
        let initializing = initializing.to_owned();
        async move {
            if !loaded.get() {
                while let Some(msg) = rx.next().await {
                    match msg {
                        InitializationMessage::Total(i) => {
                            total.set(i);
                        }
                        InitializationMessage::Progress => {
                            progress.modify(|i| i + 1);
                        }
                        _ => break,
                    }
                }
                *formats.write() = get_formats(&initialize_db(), gen);
                loaded.set(true);
                initializing.set(false);
            }
        }
    });

    let initialize = move |_| {
        cx.spawn({
            let initializing = initializing.to_owned();
            let tx = test.to_owned();
            let conn = initialize_db();
            async move {
                if !*initializing.get() {
                    initializing.set(true);
                    let _ = tokio::spawn(async move {
                        initialize_db_data(conn, gen, tx).await;
                    });
                }
            }
        });
    };

    cx.render(rsx! {
        div {
            class: "flex flex-col flex-wrap h-screen justify-center content-center text-center",
            if !initializing {
                rsx! {
                    BackButton {path: "/"}
                }
            }
            if !loaded {
                rsx! {
                    if !initializing {
                        Some(rsx! {
                            button {
                                class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                                onclick: initialize,
                                "Initialize"
                            }
                        })
                    } else if *total.get() != 0 {
                        Some(rsx! {
                            p {
                                class: "py-4",
                                "Indexing Strategy Dex: {progress.get()} / {total.get()}"
                            }
                        })
                    } else {
                        None
                    }
                }
            } else {
                rsx! {
                    div {
                        class: "grid grid-cols-3 gap-4",
                        formats.read().iter().map(|f| {
                            rsx! {
                                Link {
                                    to: "/summary?gen={gen_str}&format={f}",
                                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                                    "{f}"
                                }
                            }
                        }),
                    }
                }
            },
        }
    })
}
