use crate::components::data::PROFILE;
use crate::Theme;
use dioxus::prelude::*;

#[component]
pub fn HeaderSection(theme: Signal<Theme>) -> Element {
    rsx! {
        section {
            class: "header-section section",
            div {
                class: "container",
                div {
                    h1 { "{PROFILE.name}" }
                    p {
                        class: "header-title",
                        "{PROFILE.title}"
                    }
                }
                a {
                    class: "theme-toggle",
                    onclick: move |_| theme.set(theme().toggle()),
                    img {
                        src: "{theme().icon_theme()}",
                        alt: "Toggle theme",
                        width: "22",
                    }
                }
            }
        }
    }
}
