use crate::components::data::CERTIFICATIONS;
use crate::components::timeline_card::TimelineCard;
use dioxus::prelude::*;

#[component]
pub fn CertificationsSection() -> Element {
    rsx! {
        section {
            class: "certification-section section",
            h2 { "Certifications" }
            div {
                class: "certification-list",
                {CERTIFICATIONS.iter().map(|cert| rsx! {
                    TimelineCard {
                        card_type: "certification",
                        title: rsx! {
                            a {
                                href: "{cert.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{cert.title}"
                            }
                        },
                        meta: rsx! { "{cert.meta}" },
                    }
                })}
            }
        }
    }
}
