use crate::Theme;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
fn NavLink(label: &'static str, section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        a {
            class: "nav-link",
            onclick: move |_| async move {
                if let Some(el) = section.cloned() {
                    el.scroll_to(ScrollBehavior::Smooth).await.ok();
                }
            },
            "{label}"
        }
    }
}

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
                    NavLink { label: "Skills", section: skills_section }
                    NavLink { label: "Experience", section: experience_section }
                    NavLink { label: "Projects", section: projects_section }
                    NavLink { label: "Education", section: education_section }
                    NavLink { label: "Contact", section: contact_section }
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
