use dioxus::prelude::*;

#[component]
pub fn AboutSection() -> Element {
    let mut is_image_expanded = use_signal(|| false);

    rsx! {
        section {
            class: "about-section section",
            img {
                class: "about-photo",
                src: asset!("/assets/images/profilepic.jpg"),
                alt: "Alexander Alexandrov",
                width: "120",
                height: "120",
                onclick: move |_| {
                    is_image_expanded.set(true);
                }
            }
            div {
                class: "about-text",
                h2 { "About Me" }
                p {
                    "Software developer based in Sofia, BG. \
                    I build web applications and backend systems, mostly across the usual full‑stack technologies — \
                    and I reach for Rust when performance and reliability really matter. I care about clean naming, \
                    thoughtful abstractions, and code that's easy to change. I do my best work on collaborative, low‑ego teams. \
                    Always happy to talk software design, Rust, or wine."
                }
            }
        }

        if is_image_expanded() {
            div {
                class: "image-overlay",
                onclick: move |_| {
                    is_image_expanded.set(false);
                },
                button {
                    class: "close-button",
                    onclick: move |e| {
                        e.stop_propagation();
                        is_image_expanded.set(false);
                    },
                    "\u{00D7}"
                }
                img {
                    src: asset!("/assets/images/profilepic.jpg"),
                    alt: "Alexander Alexandrov",
                    onclick: move |e| {
                        e.stop_propagation();
                    }
                }
            }
        }
    }
}
