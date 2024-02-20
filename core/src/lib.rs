pub mod auth;

pub use fsrs::{Card, Rating};

use chrono::{DateTime, Utc};
use fsrs::FSRS;
use futures::future;
use serde::{Deserialize, Serialize};

use auth::User;

#[derive(Default)]
pub struct Config {
    pub fsrs: FSRS,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tree {
    id: u32,
    user: Option<User>,
    name: String,
    branches: Vec<Branch>,
    created_at: String,
}

impl Tree {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn branches(&self) -> Vec<Branch> {
        self.branches.clone()
    }

    pub fn get_all_stems(&self) -> Vec<Stem> {
        self.branches()
            .into_iter()
            .flat_map(|branch| branch.stems)
            .collect()
    }

    pub fn get_all_leaves(&self) -> Vec<Leaf> {
        self.branches()
            .into_iter()
            .flat_map(|branch| branch.stems.into_iter().flat_map(|stem| stem.leaves))
            .collect()
    }

    pub fn find_branch(&self, branch_id: u32) -> Option<Branch> {
        self.branches()
            .into_iter()
            .find(|branch| branch.id == branch_id)
    }

    pub fn find_stem(&self, stem_id: u32) -> Option<Stem> {
        self.get_all_stems()
            .into_iter()
            .find(|stem| stem.id == stem_id)
    }

    pub fn find_leaf(&self, leaf_id: u32) -> Option<Leaf> {
        self.get_all_leaves()
            .into_iter()
            .find(|stem| stem.id == leaf_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    id: u32,
    tree_id: u32,
    name: String,
    stems: Vec<Stem>,
    created_at: String,
}

impl Branch {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn stems(&self) -> Vec<Stem> {
        self.stems.clone()
    }

    pub fn add_stem(&mut self, stem: Stem) {
        self.stems.push(stem);
    }

    pub fn delete_stem(&mut self, id: u32) {
        self.stems = self
            .stems
            .clone()
            .into_iter()
            .filter(|stem| stem.id != id)
            .collect();
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Stem {
    id: u32,
    branch_id: u32,
    name: String,
    leaves: Vec<Leaf>,
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

    pub fn leaves(&self) -> Vec<Leaf> {
        self.leaves.clone()
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

    impl Tree {
        pub async fn get_branches(tree_id: u32, pool: &SqlitePool) -> Result<Vec<Branch>, sqlx::Error> {
            Ok(
                future::join_all(sqlx::query_as::<_, SqlBranch>("SELECT * FROM branches WHERE tree_id = ?")
                    .bind(tree_id)
                    .fetch_all(pool)
                    .await?.into_iter().map(|sql_branch| sql_branch.into_branch(pool))).await
            )
        }
    }

    #[derive(FromRow)]
    pub struct SqlTree {
        pub id: u32,
        pub user_id: i64,
        pub name: String,
        pub created_at: String,
    }

    impl SqlTree {
        pub async fn into_tree(&self, pool: &SqlitePool) -> Tree {
            Tree { id: self.id, user: User::get(self.user_id, pool).await, name: self.name.clone(), branches: Tree::get_branches(self.id, pool).await.unwrap_or_default(), created_at: self.created_at.clone() }
        }
    }

    impl Branch {
        pub async fn get_stems(branch_id: u32, pool: &SqlitePool) -> Result<Vec<Stem>, sqlx::Error> {
            Ok(
                future::join_all(sqlx::query_as::<_, SqlStem>("SELECT * FROM stems WHERE branch_id = ?")
                    .bind(branch_id)
                    .fetch_all(pool)
                    .await?.into_iter().map(|sql_stem| sql_stem.into_stem(pool))).await
            )
        }
    }

    #[derive(FromRow)]
    pub struct SqlBranch {
        pub id: u32,
        pub tree_id: u32,
        pub name: String,
        pub created_at: String,
    }

    impl SqlBranch {
        pub async fn into_branch(self, pool: &SqlitePool) -> Branch {
            Branch { id: self.id, tree_id: self.tree_id, name: self.name.clone(), stems: Branch::get_stems(self.id, pool).await.unwrap_or_default(), created_at: self.created_at.clone() }
        }
    }

    impl Stem {
        pub async fn get_leaves(stem_id: u32, pool: &SqlitePool) -> Result<Vec<Leaf>, sqlx::Error> {
            Ok(
                sqlx::query_as::<_, SqlLeaf>("SELECT * FROM leaves WHERE stem_id = ?")
                    .bind(stem_id)
                    .fetch_all(pool)
                    .await?.iter().map(|sql_leaf| sql_leaf.into_leaf()).collect()
            )
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
        pub async fn into_stem(self, pool: &SqlitePool) -> Stem {
            Stem {
                id: self.id, branch_id: self.branch_id, name: self.name.clone(), leaves: Stem::get_leaves(self.id, pool).await.unwrap_or(Vec::new()), created_at: self.created_at.clone()
            }
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
