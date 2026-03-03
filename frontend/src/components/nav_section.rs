use crate::Theme;
use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

const SECTION_IDS: &[&str] = &["skills", "experience", "projects", "education", "contact"];

#[component]
fn NavLink(
    label: &'static str,
    section: Signal<Option<Rc<MountedData>>>,
    is_active: bool,
) -> Element {
    let class = if is_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    rsx! {
        button {
            class: "{class}",
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
    active_section: Signal<String>,
    skills_section: Signal<Option<Rc<MountedData>>>,
    experience_section: Signal<Option<Rc<MountedData>>>,
    projects_section: Signal<Option<Rc<MountedData>>>,
    education_section: Signal<Option<Rc<MountedData>>>,
    contact_section: Signal<Option<Rc<MountedData>>>,
) -> Element {
    use_effect(move || {
        let Some(window) = web_sys::window() else {
            return;
        };

        let mut active = active_section.clone();
        let prev_scroll = std::cell::Cell::new(0.0_f64);
        let closure = Closure::<dyn FnMut()>::new(move || {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };
            let Some(doc_el) = document.document_element() else {
                return;
            };

            let viewport_h = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let scroll_y = window.page_y_offset().unwrap_or(0.0);
            let doc_height = doc_el.scroll_height() as f64;
            let scrolling_down = scroll_y > prev_scroll.get();
            prev_scroll.set(scroll_y);

            if scroll_y + viewport_h >= doc_height - 50.0 {
                active.set("contact".to_string());
                return;
            }

            // Scrolling down: activate when heading reaches the top (10%)
            // Scrolling up: keep section active while heading is visible (30%)
            let threshold = if scrolling_down {
                viewport_h * 0.1
            } else {
                viewport_h * 0.3
            };

            let mut active_id = String::new();
            for id in SECTION_IDS {
                if let Some(el) = document.get_element_by_id(id) {
                    if el.get_bounding_client_rect().top() < threshold {
                        active_id = id.to_string();
                    }
                }
            }

            active.set(active_id);
        });

        let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        closure.forget();
    });

    let active = active_section();

    rsx! {
        nav {
            class: "fixed-nav",
            div {
                class: "nav-bar",
                div {
                    class: "nav-links",
                    NavLink { label: "Skills", section: skills_section, is_active: active == "skills" }
                    NavLink { label: "Experience", section: experience_section, is_active: active == "experience" }
                    NavLink { label: "Projects", section: projects_section, is_active: active == "projects" }
                    NavLink { label: "Education", section: education_section, is_active: active == "education" }
                    NavLink { label: "Contact", section: contact_section, is_active: active == "contact" }
                }
                button {
                    class: "theme-toggle",
                    aria_label: "Toggle theme",
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
