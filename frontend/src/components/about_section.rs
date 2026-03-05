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
                    "I build web applications and backend systems, mostly across the usual full\u{2011}stack technologies \u{2014} \
                    and I reach for Rust when performance and reliability matter. I care about clean naming, \
                    thoughtful abstractions, and code that's easy to change. I do my best work on collaborative, low\u{2011}ego teams. \
                    Always happy to talk software design, Rust, or wine."
                }
            }
        }

        if is_image_expanded() {
            div {
                class: "image-overlay",
                tabindex: "0",
                onclick: move |_| {
                    is_image_expanded.set(false);
                },
                onkeydown: move |e: KeyboardEvent| {
                    if e.key() == Key::Escape {
                        is_image_expanded.set(false);
                    }
                },
                onmounted: move |cx| async move {
                    let _ = cx.set_focus(true).await;
                },
                button {
                    class: "close-button",
                    aria_label: "Close image",
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
