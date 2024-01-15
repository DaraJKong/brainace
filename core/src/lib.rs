use fsrs::FSRSItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deck {
    name: String,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(name: &str) -> Deck {
        Deck {
            name: name.to_string(),
            cards: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    front: String,
    back: String,
    fsrs: Option<FSRSItem>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            front: String::new(),
            back: String::new(),
            fsrs: None,
        }
    }

    pub fn front(&self) -> &String {
        &self.front
    }

    pub fn back(&self) -> &String {
        &self.back
    }
}
