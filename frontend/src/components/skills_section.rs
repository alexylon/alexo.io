use crate::components::data::SKILL_CATEGORIES;
use dioxus::prelude::*;

#[component]
pub fn SkillsSection() -> Element {
    rsx! {
        section {
            id: "skills",
            class: "skills-section section",
            tabindex: "-1",
            h2 { "Skills" }
            div {
                class: "skills-rows",
                {SKILL_CATEGORIES.iter().map(|cat| rsx! {
                    div {
                        class: "skill-row",
                        span { class: "apparatus", "{cat.name}" }
                        div {
                            class: "skills-grid",
                            {cat.skills.iter().map(|skill| rsx! {
                                span {
                                    class: "chip",
                                    "{skill}"
                                }
                            })}
                        }
                    }
                })}
            }
        }
    }
}
