use std::fmt;

use crate::{theme, widget::Element};
use chrono::Utc;
pub use fsrs::Card as FSRSCard;
use fsrs::{Rating, FSRS};
use iced::{
    widget::{column, container, text},
    Length,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deck {
    name: String,
    pub cards: Vec<Card>,
}

#[derive(Clone, Debug)]
pub enum DeckMessage {
    CardMessage(usize, CardMessage),
    Edit(usize),
}

impl Deck {
    pub fn new(name: &str) -> Deck {
        Deck {
            name: name.to_string(),
            cards: Vec::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn add_card(&mut self, front: &str, back: &str) {
        let card = Card::new(front, back);

        self.cards.push(card);
    }

    pub fn remove_card(&mut self, i: usize) {
        self.cards.remove(i);
    }

    pub fn update(&mut self, message: DeckMessage) {
        match message {
            DeckMessage::CardMessage(i, message) => {
                self.cards[i].update(message);
            }
            DeckMessage::Edit(i) => {}
        }
    }

    pub fn view(&self) -> Element<'_, DeckMessage> {
        let cards = self
            .cards
            .iter()
            .enumerate()
            .map(|(i, card)| {
                card.view()
                    .map(move |message| DeckMessage::CardMessage(i, message))
            })
            .collect();

        column(cards).spacing(10).into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    front: String,
    back: String,
    #[serde(skip)]
    state: CardState,
    fsrs: FSRSCard,
}

enum CardState {
    Hidden,
    Revealed,
}

impl Default for CardState {
    fn default() -> Self {
        Self::Hidden
    }
}

impl CardState {
    fn toggle(&mut self) {
        match self {
            Self::Hidden => *self = Self::Revealed,
            Self::Revealed => *self = Self::Hidden,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CardMessage {
    Hide,
    Reveal,
    ToggleState,
    Edit,
    Delete,
    FrontChanged(String),
    BackChanged(String),
}

impl Card {
    pub fn new(front: &str, back: &str) -> Self {
        Self {
            front: front.to_string(),
            back: back.to_string(),
            state: CardState::default(),
            fsrs: FSRSCard::new(),
        }
    }

    pub const fn revealed(&self) -> bool {
        match self.state {
            CardState::Hidden => false,
            CardState::Revealed => true,
        }
    }

    pub fn log(&self) -> &impl fmt::Debug {
        &self.fsrs.log
    }

    pub fn schedule(&mut self, fsrs: FSRS, rating: Rating) {
        let scheduled_cards = fsrs.schedule(self.fsrs.clone(), Utc::now());

        self.fsrs = scheduled_cards.select_card(rating);
    }

    pub fn update(&mut self, message: CardMessage) {
        match message {
            CardMessage::Hide => self.state = CardState::Hidden,
            CardMessage::Reveal => self.state = CardState::Revealed,
            CardMessage::ToggleState => self.state.toggle(),
            CardMessage::FrontChanged(content) => self.front = content,
            CardMessage::BackChanged(content) => self.back = content,
            _ => {}
        }
    }

    pub fn view(&self) -> Element<CardMessage> {
        let front = text(self.front.clone()).size(25);
        let back = text(self.back.clone()).size(25).style(theme::Text::Accent);

        let content: Element<_> = match self.state {
            CardState::Hidden => container(front).padding([15, 25]).into(),
            CardState::Revealed => {
                let front_container = container(front)
                    .width(Length::Fill)
                    .center_x()
                    .padding([15, 25]);
                let back_container = container(back)
                    .width(Length::Fill)
                    .center_x()
                    .padding([15, 25])
                    .style(theme::Container::BorderedFooter);

                column![front_container, back_container].into()
            }
        };

        container(content)
            .width(Length::Fill)
            .center_x()
            .style(theme::Container::Bordered)
            .into()
    }
}
