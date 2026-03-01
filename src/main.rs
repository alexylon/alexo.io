use dioxus::prelude::*;
use manganis::Asset;
use std::rc::Rc;

mod components;
use components::*;

fn main() {
    LaunchBuilder::new().launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Dark,
    Light,
}

impl Theme {
    fn css_class(&self) -> &'static str {
        match self {
            Theme::Dark => "gruvbox-dark",
            Theme::Light => "gruvbox-light",
        }
    }

    fn toggle(&self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    fn icon_theme(&self) -> Asset {
        match self {
            Theme::Dark => asset!("/assets/icons/light_mode.svg"),
            Theme::Light => asset!("/assets/icons/dark_mode.svg"),
        }
    }

    fn icon_up(&self) -> Asset {
        match self {
            Theme::Dark => asset!("/assets/icons/keyboard_arrow_up_light.svg"),
            Theme::Light => asset!("/assets/icons/keyboard_arrow_up_dark.svg"),
        }
    }
}

fn get_system_theme() -> Theme {
    use web_sys::window;

    if let Some(window) = window() {
        if let Ok(media_query) = window.match_media("(prefers-color-scheme: dark)") {
            if let Some(media_query) = media_query {
                if media_query.matches() {
                    return Theme::Dark;
                }
            }
        }
    }

    Theme::Light
}

#[component]
fn App() -> Element {
    let theme = use_signal(|| get_system_theme());

    // Reveal the page after WASM hydration to prevent flash of unstyled content
    use_effect(|| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let _ = body.set_attribute("class", "ready");
                }
            }
        }
    });

    let mut top_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let skills_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let experience_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let projects_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let education_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let contact_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/index.css")
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/gruvbox-dark.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/gruvbox-light.css"),
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&family=Podkova:wght@400..800&family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap",
        }
        document::Title { "Alexander" }

        main {
            class: "{theme().css_class()}",
            NavSection { skills_section, experience_section, projects_section, education_section, contact_section }
            div {
                class: "resume",
                onmounted: move |cx| top_element.set(Some(cx.data())),
                HeaderSection { theme }
                AboutSection {}
                SkillsSection { skills_section }
                ExperienceSection { experience_section }
                ProjectsSection { projects_section }
                EducationSection { education_section }
                CertificationsSection {}
                LanguagesSection {}
                ContactSection { contact_section }
                FooterSection {}
                ScrollToTop { top_element, theme }
            }
        }
    }
}
