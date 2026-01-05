use serde::Deserialize;
use csv::Trim;
use std::collections::HashMap;
use include_dir::Dir;
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;
#[derive(Debug, Deserialize, Clone)]
pub struct Card {
    #[serde(rename = "HASŁO", alias = "\u{feff}HASŁO")]
    pub word: String,

    #[serde(rename = "PODPOWIEDŹ", alias = "\u{feff}PODPOWIEDŹ")]
    pub hint: String,
}

pub fn load_all_categories_embedded(dir: &Dir<'static>) -> HashMap<String, Vec<Card>> {
    let mut map = HashMap::new();

    for file in dir.files() {
        if file.path().extension().and_then(|e| e.to_str()) == Some("csv") {
            let name = file.path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b';')
                .has_headers(true)
                .trim(Trim::All)
                .from_reader(file.contents());

            let mut cards = Vec::new();
            for result in reader.deserialize() {
                if let Ok(record) = result {
                    cards.push(record);
                }
            }
            map.insert(name, cards);
        }
    }
    map
}

pub fn get_random_card(
    selected_names: Vec<String>,
    all_categories: &HashMap<String, Vec<Card>>
) -> Option<Card> {
    let mut pool = Vec::new();

    for name in selected_names {
        if let Some(cards) = all_categories.get(&name) {
            pool.extend(cards.clone());
        }
    }

    let mut rng = rand::thread_rng();
    pool.choose(&mut rng).cloned()
}