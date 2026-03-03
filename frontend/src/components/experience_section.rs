use crate::components::data::EXPERIENCE_ENTRIES;
use crate::components::timeline_card::TimelineCard;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ExperienceSection(experience_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            id: "experience",
            onmounted: move |cx| experience_section.set(Some(cx.data())),
            class: "experience-section section",
            h2 { "Experience" }
            div {
                class: "experience-list",
                {EXPERIENCE_ENTRIES.iter().map(|entry| rsx! {
                    TimelineCard {
                        card_type: "experience",
                        title: rsx! { "{entry.title}" },
                        meta: rsx! {
                            span { class: "experience-company", "{entry.company}" }
                            " \u{2003}|\u{2003} "
                            "{entry.period}"
                        },
                        items: entry.responsibilities.iter().map(|s| s.to_string()).collect(),
                    }
                })}
            }
        }
    }
}
