use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

#[component]
pub fn ScrollToTop(top_element: Signal<Option<Rc<MountedData>>>) -> Element {
    let mut show_button = use_signal(|| false);

    // Use resource for proper cleanup
    let _scroll_handler = use_resource(move || async move {
        let window = match web_sys::window() {
            Some(w) => w,
            None => {
                tracing::debug!("Could not get window object");
                return None;
            }
        };

        let handle_scroll = {
            let mut show_button = show_button.clone();

            move || {
                let scroll_y = web_sys::window()
                    .and_then(|w| w.page_y_offset().ok())
                    .unwrap_or(0.0);

                show_button.set(scroll_y > 150.0);
            }
        };

        let closure = wasm_bindgen::closure::Closure::new(handle_scroll);

        if let Err(e) =
            window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
        {
            tracing::debug!("Failed to add scroll event listener: {:?}", e);
            return None;
        }

        // Initial check
        let initial_scroll = web_sys::window()
            .and_then(|w| w.page_y_offset().ok())
            .unwrap_or(0.0);

        show_button.set(initial_scroll > 150.0);

        Some(closure)
    });

    rsx! {
        button {
            class: "scroll-to-top",
            class: if show_button() { "" } else { "hidden" },
            onclick: move |_| async move {
                    if let Some(header) = top_element.cloned() {
                        header.scroll_to(ScrollBehavior::Smooth).await.ok();
                    }
                },
            img {
                src: asset!("/assets/icons/keyboard_arrow_up_light.svg"),
                alt: "Up Icon",
            }
        }
    }
}
