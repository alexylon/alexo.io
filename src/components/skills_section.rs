use crate::components::data::SKILL;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn SkillsSection(skills_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| skills_section.set(Some(cx.data())),
            class: "skills-section section",
            h2 { "Key Tech Skills" }
            div {
                class: "skills-grid",
                {SKILL.iter().map(|cat| rsx! {
                    span {
                        class: "chip",
                        "{cat.label}"
                    }
                })}
            }
        }
    }
}
