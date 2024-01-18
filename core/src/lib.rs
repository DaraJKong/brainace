use chrono::{DateTime, Utc};
use fsrs::{FSRSItem, FSRSReview, FSRS};
use serde::{Deserialize, Serialize};

pub struct Config {
    pub fsrs: FSRS,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fsrs: FSRS::new(None).unwrap(),
        }
    }
}

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

#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    front: String,
    back: String,
    fsrs_item: FSRSItem,
    last_review: Option<DateTime<Utc>>,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            front: String::new(),
            back: String::new(),
            fsrs_item: FSRSItem {
                reviews: Vec::new(),
            },
            last_review: None,
        }
    }
}

impl Card {
    pub fn new(front: &str, back: &str) -> Self {
        Self {
            front: front.to_string(),
            back: back.to_string(),
            ..Default::default()
        }
    }

    pub fn review(&mut self, rating: u32, now: DateTime<Utc>) {
        if let Some(review_time) = self.last_review {
            let delta_t = (now - review_time).num_days() as u32;
            let review = FSRSReview { rating, delta_t };

            self.fsrs_item.reviews.push(review);
        }

        self.last_review = Some(now);
    }

    pub fn front(&self) -> String {
        self.front.clone()
    }

    pub fn back(&self) -> String {
        self.back.clone()
    }

    pub fn set_front(&mut self, front: &str) {
        self.front = front.to_string();
    }

    pub fn set_back(&mut self, back: &str) {
        self.back = back.to_string();
    }
}
