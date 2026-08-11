use crate::components::data::PROJECTS;
use dioxus::prelude::*;

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section {
            id: "projects",
            class: "projects-section section",
            tabindex: "-1",
            // Matches the nav label. The rail kinds and "Source code" links
            // already say these are open source.
            h2 { "Projects" }
            div {
                class: "works-list",
                {PROJECTS.iter().map(|project| rsx! {
                    article {
                        class: "work",
                        div {
                            class: "apparatus",
                            "{project.kind}"
                        }
                        h3 {
                            class: "work-name",
                            a {
                                href: "{project.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{project.name}"
                            }
                        }
                        p {
                            class: "work-desc",
                            "{project.description}"
                        }
                        div {
                            class: "work-links",
                            a {
                                class: "work-link",
                                href: "{project.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "Source code"
                            }
                            if let Some(crate_url) = project.crate_url {
                                a {
                                    class: "work-link",
                                    href: "{crate_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "crates.io"
                                }
                            }
                            if let Some(homepage) = project.homepage {
                                {
                                    let display = homepage.trim_start_matches("https://");
                                    rsx! {
                                        a {
                                            class: "work-link",
                                            href: "{homepage}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            "{display}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}
