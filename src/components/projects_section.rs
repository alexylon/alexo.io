use crate::components::data::PROJECTS;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ProjectsSection(projects_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| projects_section.set(Some(cx.data())),
            class: "projects-section section",
            h2 { "Selected Side Projects" }
            div {
                class: "projects-grid",
                {PROJECTS.iter().map(|project| rsx! {
                    a {
                        class: "project-card",
                        href: "{project.url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        h3 {
                            class: "project-card-name",
                            "{project.name}"
                        }
                        p {
                            class: "project-card-desc",
                            "{project.description}"
                        }
                    }
                })}
            }
        }
    }
}
