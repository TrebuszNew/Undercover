mod io;
use std::{collections::HashMap, hash::Hash, num::FpCategory};
use base64::{engine::general_purpose, Engine as _};
use dioxus::{html::{button, h1, u::widows}, prelude::*};
use rand::Rng;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const MENU_CSS: Asset = asset!("/assets/menu.css");
const GAME_CSS: Asset = asset!("/assets/game.css");
const COUNTER_CSS: Asset = asset!("/assets/counter.css");

const PLAYER_IMG: Asset = asset!("/assets/player.png");
const UNDERCOVER_IMG: Asset = asset!("/assets/undercover.png");

use include_dir::{include_dir, Dir};
static CATEGORIES_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/categories");
fn data_url_png(path: &str) -> String {
    let full_path = format!("imgs/{}", path);
    
    if let Some(file) = CATEGORIES_DIR.get_file(&full_path) {
        let bytes = file.contents();
        format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(bytes)
        )
    } else {
        println!("ERROR: IMG not found: {}", full_path);
        "".to_string()
    }
}


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
    let page: Signal<&str> = use_signal(|| "menu");
    let player_count = use_signal(|| 4u8);
    let undercover_player = use_signal(|| 0u8);
    let password = use_signal(|| "".to_string());
    let hint = use_signal(|| "".to_string());
    let selected_categories = use_signal(|| HashMap::<String, bool>::new());

    let mut categories_name: Vec<String> = vec![];
    let categories = use_hook(|| io::load_all_categories_embedded(&CATEGORIES_DIR));
    for (name, cards) in categories.iter() {
        println!(" - Kategoria: {name}");
        categories_name.push((*name).clone());
        for card in cards.iter() {
            println!("   {:?}",
                card
            );
        }
    }
    
    println!("{:?}", categories_name);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: MENU_CSS }
        document::Link { rel: "stylesheet", href: GAME_CSS }
        document::Link { rel: "stylesheet", href: COUNTER_CSS }

        if *page.read() == "menu" {
            Menu {
                page,
                categories_name,
                player_count,
                undercover_player,
                all_categories: categories,
                password,
                hint,
                selected_categories,
            }
        } else if *page.read() == "game" {
            Game {
                page,
                player_count,
                undercover_player,
                password,
                hint,
            }
        } else if *page.read() == "counter" {
            Counter { page, player_count }
        }
    }
}

#[component]
pub fn Menu(
    page: Signal<&'static str>,
    categories_name: Vec<String>,
    player_count: Signal<u8>,
    undercover_player: Signal<u8>,
    all_categories: ReadOnlySignal<HashMap<String, Vec<io::Card>>>,
    password: Signal<String>,
    hint: Signal<String>,
    selected_categories: Signal<HashMap<String, bool>>,
) -> Element {
    let is_any_category_selected = *use_memo(move || {
        selected_categories.read().values().any(|&selected| selected)
    }).read();
    rsx! {
        div { id: "menu-content",
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
                for category_name in categories_name.iter() {
                    button {
                        class: "category-card",

                        onclick: {
                            let name = category_name.clone();
                            move |_| {
                                let current_state = *selected_categories.read().get(&name).unwrap_or(&false);
                                selected_categories.write().insert(name.clone(), !current_state);
                                println!("{:?}", *selected_categories.read());
                            }
                        },

                        img {
                            class: if *selected_categories.read().get(category_name).unwrap_or(&false) { "selected" } else { "not-selected" },
                            src: data_url_png(&format!("{}.png", category_name)),
                            draggable: false,
                        
                        }
                        p { "{category_name}" }
                    }
                }
            }
            div { id: "start-setting",
                button {
                    class: if is_any_category_selected { "button-active" } else { "button-unactive" },
                    disabled: !is_any_category_selected,
                    onclick: move |_| {

                        let selected_names: Vec<String> = selected_categories
                            .read()
                            .iter()
                            .filter(|(_, &active)| active)
                            .map(|(name, _)| name.clone())
                            .collect();

                        if let Some(card) = io::get_random_card(selected_names, &all_categories.read()) {
                            password.set(card.word);
                            hint.set(card.hint);

                            let pc = *player_count.read();
                            let mut rng = rand::rng();
                            undercover_player.set(rng.random_range(1..=pc));

                            page.set("game");
                        }
                    },
                    "Start"
                }
            }
        }
    }
}

#[component]
pub fn Game(
    page: Signal<&'static str>,
    player_count: Signal<u8>,
    undercover_player: Signal<u8>,
    password: Signal<String>,
    hint: Signal<String>
) -> Element {
    let mut which_player = use_signal(|| 1u8);
    let mut show_role = use_signal(|| false);
    let mut know_the_role = use_signal(|| false);

    rsx! {
        div { id: "game-content",
            div { id: "game-player-title",
                h1 { "Gracz: {which_player}" }
            }
            div { id: "game-player-rule",
                button {
                    onmousedown: move |_| {
                        show_role.set(true);
                        know_the_role.set(true);
                    },
                    onmouseup: move |_| {
                        show_role.set(false);
                    },

                    ontouchstart: move |_| {
                        show_role.set(true);
                        know_the_role.set(true);
                    },
                    ontouchend: move |_| {
                        show_role.set(false);
                    },
                    id: "game-player-rule-button",
                    class: if *show_role.read() { "game-player-rule-button-show" } else { "game-player-rule-button-hide" },
                    {
                        if *show_role.read() {

                            if *which_player.read() == *undercover_player.read() {
                                rsx! {
                                    div { id: "game-player-rule-button-text",
                                        div {
                                            h1 { "Kłamca" }
                                            img { src: UNDERCOVER_IMG, draggable: false }
                                        }
                                        strong { "Podpowiedź: {*hint.read()}" }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { id: "game-player-rule-button-text",
                                        div {
                                            h1 { "Gracz" }
                                            img { src: PLAYER_IMG, draggable: false }
                                        }
                                        strong { "Hasło: {*password.read()}" }
                                    }
                                }
                            }
                        } else {
                            rsx! { "Kliknij aby poznać rolę" }
                        }
                    }
                
                }
            }
            div { id: "game-next-button-space",
                button {
                    onclick: move |_| {
                        if *player_count.read() == *which_player.read() {
                            page.set("counter");
                        }
                        let new = *which_player.read() + 1;
                        which_player.set(new);
                        know_the_role.set(false);
                    },
                    class: if *know_the_role.read() { "button-active" } else { "button-unactive" },
                    disabled: !*know_the_role.read(),
                    "Następny"
                }
            }
        }
    }
}

#[component]
pub fn Counter(page: Signal<&'static str>, player_count: Signal<u8>) -> Element {
    let mut round_counter = use_signal(|| 1u8);
    let mut witch_player = use_signal(|| 1u8);
    rsx!(
        button {
            onclick: move |_| {
                let new = (*witch_player.read()) + 1;
                witch_player.set(new);

                if new > *player_count.read() {
                    witch_player.set(1);
                    let new_round = (*round_counter.read()) + 1;
                    round_counter.set(new_round);
                }

                if *round_counter.read() > 3 {
                    page.set("menu");
                }
            },
            id: "counter-content",
            div {
                h1 { "Runda: {round_counter}/3" }
                h2 { "Gracz: {witch_player}" }
            }
        }
    )
}