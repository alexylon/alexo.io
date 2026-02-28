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
                onclick: move |_| {
                    is_image_expanded.set(true);
                }
            }
            div {
                class: "about-text",
                h2 { "About Me" }
                p {
                    "Based in Sofia, BG, \
                    I'm a software developer who enjoys building reliable web applications and backend systems. \
                    My background is in full\u{2011}stack development across common web technologies, and I have a strong affinity for Rust when performance and reliability matter. \
                    I care about clear naming, thoughtful abstractions, and code that's easy for others to change, and I do my best work on low\u{2011}ego, collaborative teams. \
                    Always happy to connect and talk about real\u{2011}world software design, Rust, and wine."
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
