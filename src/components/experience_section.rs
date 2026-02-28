use crate::components::data::EXPERIENCE_ENTRIES;
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
                    div {
                        class: "experience-card",
                        h3 {
                            class: "experience-title",
                            "{entry.title}"
                        }
                        p {
                            class: "experience-meta",
                            "{entry.company_and_period}"
                        }
                        ul {
                            {entry.responsibilities.iter().map(|desc| rsx! {
                                li { "{desc}" }
                            })}
                        }
                    }
                })}
            }
        }
    }
}
