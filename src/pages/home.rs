use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-wrap h-screen justify-center content-center text-center",
            div {
                class: "grid grid-cols-4 gap-4",
                Link {
                    to: "/uber",
                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                    "Uber"
                }
                Link {
                    to: "/ou",
                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                    "OU"
                }
                Link {
                    to: "/uubl",
                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                    "UUBL"
                }
                Link {
                    to: "/uu",
                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                    "UU"
                }
                Link {
                    to: "/nubl",
                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                    "NUBL"
                }
                Link {
                    to: "/nu",
                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                    "NU"
                }
                Link {
                    to: "/lc",
                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                    "LC"
                }
                Link {
                    to: "/nfe",
                    class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                    "NFE"
                }
            }
        }
    })
}
