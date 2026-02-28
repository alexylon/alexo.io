use crate::components::data::EDUCATION;
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
                    div {
                        class: "education-card",
                        h3 {
                            class: "education-title",
                            "{ed.title}"
                        }
                        p {
                            class: "education-meta",
                            "{ed.institution}"
                        }
                    }
                })}
            }
        }
    }
}
