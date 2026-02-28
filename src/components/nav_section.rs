use crate::Theme;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn NavSection(
    theme: Signal<Theme>,
    skills_section: Signal<Option<Rc<MountedData>>>,
    experience_section: Signal<Option<Rc<MountedData>>>,
    projects_section: Signal<Option<Rc<MountedData>>>,
    contact_section: Signal<Option<Rc<MountedData>>>,
) -> Element {
    rsx! {
        nav {
            class: "fixed-nav",
            div {
                class: "nav-bar",
                h3 {
                    a {
                        onclick: move |_| async move {
                            if let Some(header) = skills_section.cloned() {
                                header.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Skills"
                    }
                }
                h3 {
                    a {
                        onclick: move |_| async move {
                            if let Some(header) = experience_section.cloned() {
                                header.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Experience"
                    }
                }
                h3 {
                    a {
                        onclick: move |_| async move {
                            if let Some(header) = projects_section.cloned() {
                                header.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Projects"
                    }
                }
                h3 {
                    a {
                        onclick: move |_| async move {
                            if let Some(header) = contact_section.cloned() {
                                header.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Contact"
                    }
                }
            }
        }
    }
}
