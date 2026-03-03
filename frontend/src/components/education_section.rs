use crate::components::data::EDUCATION;
use crate::components::timeline_card::TimelineCard;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn EducationSection(education_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| education_section.set(Some(cx.data())),
            class: "education-section section",
            h2 { "Education" }
            div {
                class: "education-list",
                {EDUCATION.iter().map(|ed| rsx! {
                    TimelineCard {
                        card_class: "education-card",
                        title: rsx! { "{ed.title}" },
                        meta: ed.institution.to_string(),
                    }
                })}
            }
        }
    }
}
