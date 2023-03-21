use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col flex-wrap h-screen justify-center content-center text-center",
            rsx! {
                div {
                    class: "grid grid-cols-3 gap-4",
                    Link {
                        to: "/formats?gen=rb",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 1"
                    }
                    Link {
                        to: "/formats?gen=gs",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 2"
                    }
                    Link {
                        to: "/formats?gen=rs",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 3"
                    }
                    Link {
                        to: "/formats?gen=dp",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 4"
                    }
                    Link {
                        to: "/formats?gen=bw",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 5"
                    }
                    Link {
                        to: "/formats?gen=xy",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 6"
                    }
                    Link {
                        to: "/formats?gen=sm",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 7"
                    }
                    Link {
                        to: "/formats?gen=ss",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 8"
                    }
                    Link {
                        to: "/formats?gen=sv",
                        class: "bg-blue-500 py-16 px-32 rounded-lg shadow-lg transition duration-300 hover:cursor-pointer hover:bg-blue-300",
                        "Gen 9"
                    }
                }
            }
        }
    })
}
