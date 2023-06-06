use crate::components::BackButton;
use crate::{
    get_formats, initialize_db, initialize_db_data, table_exists, update_db_data,
    InitializationMessage,
};
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
        if *loaded.get() {
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
            while let Some(msg) = rx.next().await {
                match msg {
                    InitializationMessage::Total(i) => {
                        total.set(i);
                    }
                    InitializationMessage::Progress => {
                        progress.modify(|i| i + 1);
                    }
                    _ => {
                        *formats.write() = get_formats(&initialize_db(), gen);
                        loaded.set(true);
                        initializing.set(false);
                    }
                }
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
                    tokio::spawn(async move {
                        initialize_db_data(conn, gen, tx).await;
                    });
                }
            }
        });
    };

    let update = move |_| {
        cx.spawn({
            let loaded = loaded.to_owned();
            let initializing = initializing.to_owned();
            let tx = test.to_owned();
            let conn = initialize_db();
            async move {
                loaded.set(false);
                if !*initializing.get() {
                    initializing.set(true);
                    tokio::spawn(async move {
                        update_db_data(conn, gen, tx).await;
                    });
                }
            }
        })
    };

    cx.render(rsx! {
        div {
            class: "min-h-screen",
            div {
                class: "flex flex-col flex-wrap py-4 content-center text-center",
                if !initializing {
                    rsx! {
                        BackButton {path: "/"},
                        if **loaded {
                            rsx! {
                                button {
                                    class: "absolute py-4 px-6 right-4 top-4 text-center bg-red-300 rounded-lg shadow-md transition duration-300 hover:cursor-pointer hover:bg-red-100",
                                    onclick: update,
                                    "Update"
                                }
                            }
                        }

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
        }
    })
}
