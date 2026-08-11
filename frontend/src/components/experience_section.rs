use crate::components::data::EXPERIENCE_ENTRIES;
use crate::components::entry_card::EntryCard;
use dioxus::prelude::*;

#[component]
pub fn ExperienceSection() -> Element {
    rsx! {
        section {
            id: "experience",
            class: "experience-section section",
            // A focus destination for nav jumps, not a tab stop.
            tabindex: "-1",
            h2 { "Experience" }
            div {
                class: "entry-list",
                {EXPERIENCE_ENTRIES.iter().map(|entry| rsx! {
                    EntryCard {
                        apparatus: rsx! { "{entry.period}" },
                        title: rsx! { "{entry.title}" },
                        org: rsx! { "{entry.company}" },
                        items: entry.responsibilities.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                    }
                })}
            }
        }
    }
}
