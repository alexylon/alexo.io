use crate::components::data::PROFILE;
use dioxus::prelude::*;

#[component]
pub fn HeaderSection() -> Element {
    let now = js_sys::Date::new_0();
    let years = now.get_full_year().saturating_sub(2019) - if now.get_month() < 8 { 1 } else { 0 };

    rsx! {
        section {
            class: "header-section section",
            h1 { "{PROFILE.name}" }
            p {
                class: "header-title",
                "{PROFILE.title} \u{2022} Sofia, BG \u{2022} {years}+ years"
            }
        }
    }
}
