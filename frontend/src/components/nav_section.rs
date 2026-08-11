use crate::Theme;
use dioxus::prelude::*;

// Document order matters: the last section past the probe line wins.
#[cfg(target_arch = "wasm32")]
const SECTION_IDS: &[&str] = &["experience", "projects", "skills", "education", "contact"];

const NAV_ITEMS: &[(&str, &str)] = &[
    ("experience", "Experience"),
    ("projects", "Projects"),
    ("skills", "Skills"),
    ("education", "Education"),
    ("contact", "Contact"),
];

/// How long a nav click holds the highlight before the scroll-spy resumes.
/// Covers the scroll it starts; after that no scroll events fire until the
/// visitor moves the page themselves.
const CLICK_SETTLE_MS: f64 = 900.0;

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    0.0
}

/// Anchors rather than buttons, so deep links and open-in-new-tab work and the
/// nav runs before WASM loads. Once loaded the handler makes the same move
/// without the history entry.
#[component]
fn NavLink(
    id: &'static str,
    label: &'static str,
    is_active: bool,
    mut active_section: Signal<String>,
    mut spy_muted_until: Signal<f64>,
) -> Element {
    let class = if is_active {
        "nav-link active"
    } else {
        "nav-link"
    };
    rsx! {
        a {
            class: "{class}",
            href: "#{id}",
            aria_current: if is_active { "true" } else { "false" },
            // A click states where the visitor wants to be, so it wins over the
            // spy. Education and Contact land on the same offset, and scroll
            // position alone cannot tell those two apart.
            onclick: move |evt: MouseEvent| {
                // Cmd/ctrl/shift-click opens a new tab or window: browser's job.
                if !evt.modifiers().is_empty() {
                    return;
                }
                evt.prevent_default();
                active_section.set(id.to_string());
                spy_muted_until.set(now_ms() + CLICK_SETTLE_MS);
                crate::components::go_to_section(id);
            },
            "{label}"
        }
    }
}

#[component]
pub fn NavSection(theme: Signal<Theme>, active_section: Signal<String>) -> Element {
    let spy_muted_until = use_signal(|| 0.0_f64);

    // Highlights the section in view. Client only; the server build never
    // scrolls, so `active_section` stays empty there.
    #[cfg(target_arch = "wasm32")]
    {
        use crate::components::ScrollCleanup;
        use std::rc::Rc;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let _cleanup: Option<Rc<ScrollCleanup>> = use_hook(|| {
            let window = web_sys::window()?;
            let mut active = active_section.clone();

            let closure = Closure::<dyn FnMut()>::new(move || {
                // Let a click's own scroll finish without overruling it.
                if now_ms() < spy_muted_until() {
                    return;
                }
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

                // The page ends before Contact can reach the probe line, so pin
                // it near the foot.
                if scroll_y + viewport_h >= doc_height - viewport_h * 0.08 {
                    active.set("contact".to_string());
                    return;
                }

                // A section takes over once its top passes this line. Keep the
                // line above the smallest gap between two sections: any lower
                // and the next section's top crosses at the same moment, so the
                // shorter one is never active.
                let threshold = viewport_h * 0.35;

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

            window
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .ok()?;

            Some(Rc::new(ScrollCleanup { closure }))
        });
    }

    let active = active_section();
    // No aria-pressed: it would announce a state that contradicts the label.
    let theme_label = match theme() {
        Theme::Dark => "Switch to light theme",
        Theme::Light => "Switch to dark theme",
    };

    rsx! {
        nav {
            class: "fixed-nav",
            aria_label: "Sections",
            div {
                class: "nav-bar",
                a {
                    class: "nav-wordmark",
                    href: "#top",
                    onclick: move |evt: MouseEvent| {
                        if !evt.modifiers().is_empty() {
                            return;
                        }
                        evt.prevent_default();
                        crate::components::go_to_top();
                    },
                    span { class: "nav-wordmark-full", "Alexander Alexandrov" }
                    span { class: "nav-wordmark-short", "A\u{00B7}A" }
                }
                div {
                    class: "nav-links",
                    {NAV_ITEMS.iter().map(|(id, label)| rsx! {
                        NavLink { id: *id, label: *label, is_active: active == *id, active_section, spy_muted_until }
                    })}
                }
                button {
                    r#type: "button",
                    class: "theme-toggle",
                    aria_label: "{theme_label}",
                    onclick: move |_| {
                        let new_theme = theme().toggle();
                        // CSS reads the theme off the class on <main>.
                        theme.set(new_theme);
                        crate::theme_store::save_theme(new_theme);
                    },
                    // Inline SVG inherits currentColor, so one shape covers
                    // both themes with no extra request.
                    svg {
                        class: "icon",
                        view_box: "0 -960 960 960",
                        width: "20",
                        height: "20",
                        fill: "currentColor",
                        "aria-hidden": "true",
                        "focusable": "false",
                        if matches!(theme(), Theme::Dark) {
                            // Sun
                            path { d: "M480-360q50 0 85-35t35-85q0-50-35-85t-85-35q-50 0-85 35t-35 85q0 50 35 85t85 35Zm0 80q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM200-440H40v-80h160v80Zm720 0H760v-80h160v80ZM440-760v-160h80v160h-80Zm0 720v-160h80v160h-80ZM256-650l-101-97 57-59 96 100-52 56Zm492 496-97-101 53-55 101 97-57 59Zm-98-550 97-101 59 57-100 96-56-52ZM154-212l101-97 55 53-97 101-59-57Zm326-268Z" }
                        } else {
                            // Moon
                            path { d: "M480-120q-150 0-255-105T120-480q0-150 105-255t255-105q14 0 27.5 1t26.5 3q-41 29-65.5 75.5T444-660q0 90 63 153t153 63q55 0 101-24.5t75-65.5q2 13 3 26.5t1 27.5q0 150-105 255T480-120Zm0-80q88 0 158-48.5T740-375q-20 5-40 8t-40 3q-123 0-209.5-86.5T364-660q0-20 3-40t8-40q-78 32-126.5 102T200-480q0 116 82 198t198 82Zm-10-270Z" }
                        }
                    }
                }
            }
        }
    }
}
