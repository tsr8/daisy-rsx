use dioxus::prelude::*;

#[component]
pub fn Hero(title: String, subtitle: String,
    cta: String,
    cta_link: String,) -> Element {
    rsx! {
        div {
            class: "hero bg-base-200 min-h-screen",
            div {
                class: "hero-content text-center",
                div {
                    class: "max-w-md",
                    h1 {
                        class: "text-5xl font-bold",
                        "{title}"
                    }
                    p {
                        class: "py-6",
                        "{subtitle}"
                    }
                    div {
                        class: "flex gap-2 justify-center",
                        a {
                            class: "btn btn-primary",
                            href: cta_link,
                            "{cta}"
                        }
                    }
                }
            }
        }
    }
}
