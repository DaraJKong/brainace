use chrono::{DateTime, Utc};
use fsrs::{Card, Rating, FSRS};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Config {
    pub fsrs: FSRS,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stem {
    pub name: String,
    pub leaves: Vec<Leaf>,
}

impl Stem {
    pub fn new(name: &str) -> Stem {
        Stem {
            name: name.to_string(),
            leaves: Vec::new(),
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Leaf {
    quiz: String,
    answer: String,
    card: Card,
    #[serde(skip)]
    pub revealed: bool,
}

impl Default for Leaf {
    fn default() -> Self {
        Self {
            quiz: String::new(),
            answer: String::new(),
            card: Card::new(),
            revealed: false,
        }
    }
}

impl Leaf {
    pub fn new(front: &str, back: &str) -> Self {
        Self {
            quiz: front.to_string(),
            answer: back.to_string(),
            ..Default::default()
        }
    }

    pub fn review(&mut self, config: Config, rating: Rating, now: DateTime<Utc>) {
        let scheduled_cards = config.fsrs.schedule(self.card.clone(), now);
        self.card = scheduled_cards.select_card(rating);
    }

    pub fn quiz(&self) -> String {
        self.quiz.clone()
    }

    pub fn answer(&self) -> String {
        self.answer.clone()
    }

    pub fn set_quiz(&mut self, front: &str) {
        self.quiz = front.to_string();
    }

    pub fn set_answer(&mut self, back: &str) {
        self.answer = back.to_string();
    }
}
