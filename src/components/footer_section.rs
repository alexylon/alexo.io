use dioxus::prelude::*;

#[component]
pub fn FooterSection() -> Element {
    rsx! {
        hr { class: "dashed" }

        footer {
            class: "footer-section",
            p {"Hosted on my Raspberry Pi"}
            img { src: "https://forthebadge.com/images/badges/made-with-rust.svg" }
            br {}
            p { "© 2026 Alexander Alexandrov" }
            p {
                a {
                    href: "https://github.com/alexylon/alexandroff.dev",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "source code"
                }
            }
        }
    }
}
