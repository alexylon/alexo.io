use crate::components::data::EXPERIENCE_ENTRIES;
use crate::components::timeline_card::TimelineCard;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ExperienceSection(experience_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| experience_section.set(Some(cx.data())),
            class: "experience-section section",
            h2 { "Experience" }
            div {
                class: "experience-list",
                {EXPERIENCE_ENTRIES.iter().map(|entry| rsx! {
                    TimelineCard {
                        card_class: "experience-card",
                        title: rsx! { "{entry.title}" },
                        meta: entry.company_and_period.to_string(),
                        items: entry.responsibilities.iter().map(|s| s.to_string()).collect(),
                    }
                })}
            }
        }
    }
}
