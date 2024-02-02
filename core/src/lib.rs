pub mod auth;

use chrono::{DateTime, Utc};
pub use fsrs::Card;
use fsrs::{Rating, FSRS};
use serde::{Deserialize, Serialize};

use auth::User;

#[derive(Default)]
pub struct Config {
    pub fsrs: FSRS,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stem {
    name: String,
    leaves: Vec<Leaf>,
}

impl Stem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            leaves: Vec::new(),
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Leaf {
    id: u32,
    user: Option<User>,
    front: String,
    back: String,
    created_at: String,
    card: Card,
}

impl Leaf {
    pub fn new(front: &str, back: &str, now: DateTime<Utc>) -> Self {
        Self {
            front: front.to_string(),
            back: back.to_string(),
            created_at: now.to_string(),
            ..Default::default()
        }
    }

    pub fn review(&mut self, config: &Config, rating: Rating, now: DateTime<Utc>) {
        let scheduled_cards = config.fsrs.schedule(self.card.clone(), now);
        self.card = scheduled_cards.select_card(rating);
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn front(&self) -> String {
        self.front.clone()
    }

    pub fn back(&self) -> String {
        self.back.clone()
    }

    pub fn created_at(&self) -> String {
        self.created_at.clone()
    }

    pub fn set_front(&mut self, front: &str) {
        self.front = front.to_string();
    }

    pub fn set_back(&mut self, back: &str) {
        self.back = back.to_string();
    }
}

cfg_if::cfg_if! { if #[cfg(feature = "auth")] {
    use sqlx::SqlitePool;

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlLeaf {
        id: u32,
        user_id: i64,
        front: String,
        back: String,
        created_at: String,
        card: sqlx::types::Json<Card>,
    }

    impl SqlLeaf {
        pub async fn into_leaf(self, pool: &SqlitePool) -> Leaf {
            Leaf {
                id: self.id,
                user: User::get(self.user_id, pool).await,
                front: self.front,
                back: self.back,
                created_at: self.created_at,
                card: self.card.0
            }
        }
    }
}}
