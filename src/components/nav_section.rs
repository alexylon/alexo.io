use crate::Theme;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn NavSection(
    theme: Signal<Theme>,
    skills_section: Signal<Option<Rc<MountedData>>>,
    experience_section: Signal<Option<Rc<MountedData>>>,
    projects_section: Signal<Option<Rc<MountedData>>>,
    education_section: Signal<Option<Rc<MountedData>>>,
    contact_section: Signal<Option<Rc<MountedData>>>,
) -> Element {
    rsx! {
        nav {
            class: "fixed-nav",
            div {
                class: "nav-bar",
                div {
                    class: "nav-links",
                    a {
                        class: "nav-link",
                        onclick: move |_| async move {
                            if let Some(el) = skills_section.cloned() {
                                el.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Skills"
                    }
                    a {
                        class: "nav-link",
                        onclick: move |_| async move {
                            if let Some(el) = experience_section.cloned() {
                                el.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Experience"
                    }
                    a {
                        class: "nav-link",
                        onclick: move |_| async move {
                            if let Some(el) = projects_section.cloned() {
                                el.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Projects"
                    }
                    a {
                        class: "nav-link",
                        onclick: move |_| async move {
                            if let Some(el) = education_section.cloned() {
                                el.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Education"
                    }
                    a {
                        class: "nav-link",
                        onclick: move |_| async move {
                            if let Some(el) = contact_section.cloned() {
                                el.scroll_to(ScrollBehavior::Smooth).await.ok();
                            }
                        },
                        "Contact"
                    }
                }
                a {
                    class: "theme-toggle",
                    onclick: move |_| theme.set(theme().toggle()),
                    img {
                        src: "{theme().icon_theme()}",
                        alt: "Toggle theme",
                        width: "22",
                    }
                }
            }
        }
    }
}
