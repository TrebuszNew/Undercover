use dioxus::prelude::*;

const MENU_CSS: Asset = asset!("/assets/menu.css");
const IMG: Asset = asset!("/assets/images.png");

#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::wry::dpi::PhysicalSize;
    use dioxus::desktop::{Config, WindowBuilder};

    let scale = 1.0;
    let window = WindowBuilder::new()
        .with_title("Undercover")
        .with_inner_size(PhysicalSize::new(412.0 * scale, 915.0 * scale));

    let config = Config::new()
        .with_window(window)
        .with_menu(None);

    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[cfg(feature = "mobile")]
fn main() {
    LaunchBuilder::mobile().launch(App);
}

#[component]
fn App() -> Element {
    let page = use_signal(|| "menu");
    let player_count = use_signal(|| 4u8); // Explicit u8

    rsx! {
        document::Link { rel: "stylesheet", href: MENU_CSS }

        // Wrapped in a parent container or fragment
        if *page.read() == "menu" {
            Menu { page, player_count }
        } else if *page.read() == "game" {
            Game { page, player_count }
        }
    }
}

#[component]
pub fn Menu(page: Signal<&'static str>, player_count: Signal<u8>) -> Element {
    rsx! {
        div { id: "main-content",
            div { id: "player-count-setting",
                h1 { "Players" }
                div { id: "player-count",
                    button {
                        onclick: move |_| {
                            if *player_count.read() > 3 {
                                player_count -= 1;
                            }
                        },
                        class: if *player_count.read() > 3 { "button-active" } else { "button-unactive" },
                        "<"
                    }
                    h1 { "{player_count}" }
                    button {
                        onclick: move |_| {
                            if *player_count.read() < 9 {
                                player_count += 1;
                            }
                        },
                        class: if *player_count.read() < 9 { "button-active" } else { "button-unactive" },
                        ">"
                    }
                }
            }
            div { id: "categories-setting",
                for i in 0..99 {
                    div { class: "category-card",
                        img { src: IMG, draggable: false }
                        p { "text: {i}" }
                    }
                }
            }
            div { id: "start-setting",
                button {
                    class: "button-active",
                    onclick: move |_| {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        page.set("game");
                    },
                    "Start"
                }
            }
        }
    }
}

#[component]
pub fn Game(page: Signal<&'static str>, player_count: Signal<u8>) -> Element {
    rsx! {
        p { "TODO" }
    }
}