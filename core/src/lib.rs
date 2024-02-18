pub mod auth;

pub use fsrs::{Card, Rating};

use chrono::{DateTime, Utc};
use fsrs::FSRS;
use serde::{Deserialize, Serialize};

use auth::User;

#[derive(Default)]
pub struct Config {
    pub fsrs: FSRS,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Branch {
    id: u32,
    user: Option<User>,
    name: String,
    created_at: String,
}

impl Branch {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Stem {
    id: u32,
    branch_id: u32,
    name: String,
    created_at: String,
}

impl Stem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Leaf {
    id: u32,
    stem_id: u32,
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

    pub fn stem_id(&self) -> u32 {
        self.stem_id
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

    pub fn card(&self) -> &Card {
        &self.card
    }

    pub fn set_front(&mut self, front: &str) {
        self.front = front.to_string();
    }

    pub fn set_back(&mut self, back: &str) {
        self.back = back.to_string();
    }
}

cfg_if::cfg_if! { if #[cfg(feature = "auth")] {
    use sqlx::{FromRow, SqlitePool};

    #[derive(FromRow)]
    pub struct SqlBranch {
        pub id: u32,
        pub user_id: i64,
        pub name: String,
        pub created_at: String,
    }

    impl SqlBranch {
        pub async fn into_branch(&self, pool: &SqlitePool) -> Branch {
            Branch { id: self.id, user: User::get(self.user_id, pool).await, name: self.name.clone(), created_at: self.created_at.clone() }
        }
    }

    impl Stem {
        pub async fn get_leaves(stem_id: i64, pool: &SqlitePool) -> Option<Vec<Leaf>> {
            sqlx::query_as::<_, SqlLeaf>("SELECT * FROM leaves WHERE stem_id = ?")
                .bind(stem_id)
                .fetch_all(pool)
                .await.ok().map(|sql_leaves| sql_leaves.iter().map(|sql_leaf| sql_leaf.into_leaf()).collect())
        }
    }

    #[derive(FromRow)]
    pub struct SqlStem {
        pub id: u32,
        pub branch_id: u32,
        pub name: String,
        pub created_at: String,
    }

    impl SqlStem {
        pub fn into_stem(&self) -> Stem {
            Stem { id: self.id, branch_id: self.branch_id, name: self.name.clone(), created_at: self.created_at.clone() }
        }
    }

    #[derive(FromRow, Clone)]
    pub struct SqlLeaf {
        id: u32,
        stem_id: u32,
        front: String,
        back: String,
        card: sqlx::types::Json<Card>,
        created_at: String,
    }

    impl SqlLeaf {
        pub fn into_leaf(&self) -> Leaf {
            Leaf {
                id: self.id,
                stem_id: self.stem_id,
                front: self.front.clone(),
                back: self.back.clone(),
                card: self.card.0.clone(),
                created_at: self.created_at.clone()
            }
        }
    }
}}
