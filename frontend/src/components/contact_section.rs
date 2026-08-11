use crate::components::data::CONTACT_LINKS;
use dioxus::prelude::*;

#[component]
pub fn ContactSection() -> Element {
    let resume_href = asset!("/assets/docs/Resume_Alexander_Alexandrov.pdf").to_string();
    let email = CONTACT_LINKS
        .iter()
        .find(|link| link.href.starts_with("mailto:"));

    rsx! {
        section {
            id: "contact",
            class: "contact-section section",
            tabindex: "-1",
            h2 { "Contact" }
            p {
                class: "contact-intro",
                "Always happy to discuss software design, Rust, or wine."
            }
            if let Some(email) = email {
                a {
                    class: "contact-email",
                    href: "{email.href}",
                    {email.href.trim_start_matches("mailto:")}
                }
            }
            div {
                class: "contact-links",
                {CONTACT_LINKS.iter()
                    .filter(|link| !link.href.starts_with("mailto:"))
                    .map(|link| {
                        let is_resume = link.download.is_some();
                        let href = if is_resume {
                            resume_href.clone()
                        } else {
                            link.href.to_string()
                        };
                        let class = if is_resume { "contact-resume" } else { "contact-link" };
                        rsx! {
                            a {
                                class: "{class}",
                                href: "{href}",
                                target: link.target,
                                rel: link.rel,
                                download: link.download,
                                "{link.label}"
                            }
                        }
                    })}
            }
        }
    }
}
