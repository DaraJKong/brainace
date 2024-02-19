use chrono::{Datelike, Utc};
use fsrs::State;

use crate::Leaf;

pub fn filter_due_today(leaves: Vec<Leaf>) -> Vec<Leaf> {
    leaves.clone().into_iter().filter(today_filter).collect()
}

pub fn filter_due_now(leaves: Vec<Leaf>) -> Vec<Leaf> {
    leaves.clone().into_iter().filter(now_filter).collect()
}

pub fn count_due_today(leaves: Vec<Leaf>) -> usize {
    leaves.into_iter().filter(today_filter).count()
}

pub fn count_due_now(leaves: Vec<Leaf>) -> usize {
    leaves.into_iter().filter(now_filter).count()
}

fn today_filter(leaf: &Leaf) -> bool {
    leaf.card().due.num_days_from_ce() <= Utc::now().num_days_from_ce()
}

fn now_filter(leaf: &Leaf) -> bool {
    leaf.card().due.timestamp_millis() <= Utc::now().timestamp_millis()
}

pub fn count_states(leaves: Vec<Leaf>) -> (u32, u32, u32, u32) {
    leaves.iter().fold(
        (0u32, 0u32, 0u32, 0u32),
        |(new, learning, review, relearning), leaf| match leaf.card().state {
            State::New => (new + 1, learning, review, relearning),
            State::Learning => (new, learning + 1, review, relearning),
            State::Review => (new, learning, review + 1, relearning),
            State::Relearning => (new, learning, review, relearning + 1),
        },
    )
}
