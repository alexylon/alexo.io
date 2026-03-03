use dioxus::prelude::*;

#[component]
pub fn TimelineCard(
    card_class: &'static str,
    title: Element,
    meta: String,
    #[props(default)] items: Vec<String>,
) -> Element {
    let title_class = card_class.replace("-card", "-title");
    let meta_class = card_class.replace("-card", "-meta");

    rsx! {
        div {
            class: "{card_class}",
            h3 {
                class: "{title_class}",
                {title}
            }
            p {
                class: "{meta_class}",
                "{meta}"
            }
            if !items.is_empty() {
                ul {
                    {items.iter().map(|item| rsx! {
                        li { "{item}" }
                    })}
                }
            }
        }
    }
}
