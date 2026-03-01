use crate::components::data::PROFILE;
use dioxus::prelude::*;

#[component]
pub fn HeaderSection() -> Element {
    rsx! {
        section {
            class: "header-section section",
            h1 { "{PROFILE.name}" }
            p {
                class: "header-title",
                "{PROFILE.title}"
            }
        }
    }
}
