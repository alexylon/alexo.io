use dioxus::prelude::*;

#[component]
pub fn FooterSection() -> Element {
    rsx! {
        footer {
            class: "footer-section",
            p { "Hosted on a Raspberry Pi" }
            p {
                class: "footer-badge",
                "Built with Rust"
            }
            p { "\u{00A9} 2026 Alexander Alexandrov" }
            p {
                a {
                    href: "https://github.com/alexylon/alexo.io",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Source Code"
                }
            }
        }
    }
}
