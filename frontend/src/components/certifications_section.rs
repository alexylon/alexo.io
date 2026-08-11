use crate::components::data::CERTIFICATIONS;
use crate::components::entry_card::EntryCard;
use dioxus::prelude::*;

#[component]
pub fn CertificationsSection() -> Element {
    rsx! {
        section {
            // `section-coda` gathers the short closing sections into one
            // cluster.
            id: "certification",
            class: "certification-section section section-coda",
            h2 { "Certification" }
            div {
                class: "entry-list entry-list-tight",
                {CERTIFICATIONS.iter().map(|cert| rsx! {
                    EntryCard {
                        apparatus: rsx! { "{cert.meta}" },
                        // Plain, like its Education neighbours. Colour reads as
                        // rank before it reads as a link, and a certificate must
                        // not outweigh the degrees above it, so the verification
                        // sits in its own link below.
                        title: rsx! { "{cert.title}" },
                        links: rsx! {
                            a {
                                class: "work-link",
                                href: "{cert.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "Verify on Credly"
                            }
                        },
                    }
                })}
            }
        }
    }
}
