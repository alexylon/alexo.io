use crate::components::data::{ContactLinkHref, CONTACT_LINKS};
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ContactSection(contact_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| contact_section.set(Some(cx.data())),
            class: "contact-section section",
            h2 { "Contact" }
            div {
                class: "contact-grid",
                {CONTACT_LINKS.iter().map(|link| {
                    let css_class = format!(
                        "contact-card {}",
                        link.label.to_lowercase()
                    );
                    match &link.href {
                        ContactLinkHref::Plain(href) => rsx! {
                            a {
                                class: "{css_class}",
                                href: "{href}",
                                target: link.target.unwrap_or(""),
                                rel: link.rel.unwrap_or(""),
                                "{link.label}"
                            }
                        },
                        // ContactLinkHref::ResumeAsset => rsx! {
                        //     a {
                        //         class: "{css_class}",
                        //         download: link.download.unwrap_or(""),
                        //         "{link.label}"
                        //     }
                        // },
                    }
                })}
            }
        }
    }
}
