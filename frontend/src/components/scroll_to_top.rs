use dioxus::prelude::*;

/// An anchor, so it works on the prerendered page before WASM loads. Only the
/// show/hide needs the client; without it the control stays hidden.
#[component]
pub fn ScrollToTop() -> Element {
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused_mut))]
    let mut show_button = use_signal(|| false);

    // Registered once; ScrollCleanup removes it on drop.
    #[cfg(target_arch = "wasm32")]
    {
        use crate::components::ScrollCleanup;
        use std::rc::Rc;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let _cleanup: Option<Rc<ScrollCleanup>> = use_hook(|| {
            let window = web_sys::window()?;

            let handle_scroll = {
                let mut show_button = show_button.clone();
                move || {
                    let scroll_y = web_sys::window()
                        .and_then(|w| w.page_y_offset().ok())
                        .unwrap_or(0.0);
                    show_button.set(scroll_y > 150.0);
                }
            };

            let closure = Closure::new(handle_scroll);
            window
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .ok()?;

            let initial_scroll = window.page_y_offset().unwrap_or(0.0);
            show_button.set(initial_scroll > 150.0);

            Some(Rc::new(ScrollCleanup { closure }))
        });
    }

    rsx! {
        a {
            class: "scroll-to-top",
            class: if show_button() { "" } else { "hidden" },
            href: "#top",
            aria_label: "Back to top",
            onclick: move |evt: MouseEvent| {
                if !evt.modifiers().is_empty() {
                    return;
                }
                evt.prevent_default();
                crate::components::go_to_top();
            },
            svg {
                class: "icon",
                view_box: "0 -960 960 960",
                width: "22",
                height: "22",
                fill: "currentColor",
                "aria-hidden": "true",
                "focusable": "false",
                path { d: "M480-528 296-344l-56-56 240-240 240 240-56 56-184-184Z" }
            }
        }
    }
}
