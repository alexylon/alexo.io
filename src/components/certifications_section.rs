use dioxus::prelude::*;

#[component]
pub fn CertificationsSection() -> Element {
    rsx! {
        section {
            class: "certification-section section",
            h2 { "Certifications" }
            div {
                class: "certification-list",
                div {
                    class: "certification-card",
                    h3 {
                        class: "certification-title",
                        a {
                            href: "https://www.credly.com/badges/13918dd1-e5ad-4e81-96c6-95fcb6fb8b3c",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Oracle Certified Associate, Java SE 8 Programmer"
                        }
                    }
                    p {
                        class: "certification-meta",
                        "Jan 2019"
                    }
                }
            }
        }
    }
}
