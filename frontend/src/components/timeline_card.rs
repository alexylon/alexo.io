use dioxus::prelude::*;

#[component]
pub fn TimelineCard(
    card_type: &'static str,
    title: Element,
    meta: Element,
    #[props(default)] items: Vec<String>,
) -> Element {
    rsx! {
        div {
            class: "{card_type}-card",
            h3 {
                class: "{card_type}-title",
                {title}
            }
            p {
                class: "{card_type}-meta",
                {meta}
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
