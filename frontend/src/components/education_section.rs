use crate::components::data::EDUCATION;
use crate::components::entry_card::EntryCard;
use dioxus::prelude::*;

#[component]
pub fn EducationSection() -> Element {
    rsx! {
        section {
            id: "education",
            class: "education-section section",
            tabindex: "-1",
            h2 { "Education" }
            div {
                class: "entry-list entry-list-tight",
                {EDUCATION.iter().map(|ed| rsx! {
                    EntryCard {
                        // The rail carries the level, so a doctorate and a
                        // short course are told apart at a glance.
                        apparatus: rsx! { "{ed.kind}" },
                        title: rsx! { "{ed.title}" },
                        org: rsx! { "{ed.institution}" },
                    }
                })}
            }
        }
    }
}
