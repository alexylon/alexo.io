use crate::components::data::LANGUAGES;
use dioxus::prelude::*;

#[component]
pub fn LanguagesSection() -> Element {
    rsx! {
        section {
            id: "languages",
            class: "languages-section section section-coda",
            h2 { "Spoken Languages" }
            // Positioning context for the rail label; anchored to the section
            // it would collide with the h2.
            div {
                class: "lang-block",
                // The codes mean nothing outside Europe without this.
                span { class: "apparatus", "CEFR levels" }
                p {
                    class: "lang-line",
                    {LANGUAGES.iter().enumerate().map(|(i, lang)| rsx! {
                        if i > 0 {
                            span { class: "sep", aria_hidden: "true", "\u{00B7}" }
                        }
                        // Keeps "Russian B1" from breaking mid-word at narrow
                        // widths.
                        span {
                            class: "lang-item",
                            "{lang.name}"
                            span {
                                class: "lang-level",
                                "{lang.level}"
                            }
                        }
                    })}
                }
            }
        }
    }
}
