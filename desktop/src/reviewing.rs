use std::fmt;

use crate::{
    action, action_btn, icon_btn, icon_eye, icon_eye_off, icon_pencil, icon_plus, icon_trash,
    theme, widget::Element,
};
use iced::{
    widget::{column, container, horizontal_space, row, text, text_input},
    Alignment, Length,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deck {
    name: String,
    pub cards: Vec<Card>,
    #[serde(skip)]
    editing_name: bool,
}

#[derive(Clone, Debug)]
pub enum DeckMessage {
    NewCard(usize),
    EditName,
    NameChanged(String),
    FinishEditName,
    CardMessage(usize, CardMessage),
    FrontChanged(String),
    BackChanged(String),
    Review,
}

impl Deck {
    pub fn new(name: &str) -> Deck {
        Deck {
            name: name.to_string(),
            cards: Vec::new(),
            editing_name: false,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn update(&mut self, message: DeckMessage) {
        match message {
            DeckMessage::NewCard(i) => {
                let card = Card::new("", "");
                self.cards.push(card);
            }
            DeckMessage::EditName => {
                self.editing_name = true;
            }
            DeckMessage::NameChanged(content) => {
                self.name = content;
            }
            DeckMessage::FinishEditName => {
                self.editing_name = false;
            }
            DeckMessage::CardMessage(i, message) => match message {
                CardMessage::Delete => {
                    self.cards.remove(i);
                }
                _ => {
                    self.cards[i].update(message);
                }
            },
            _ => (),
        }
    }

    pub fn view(&self) -> Element<'_, DeckMessage> {
        let deck_name: Element<_> = if self.editing_name {
            text_input("Name the deck...", &self.name)
                .on_input(DeckMessage::NameChanged)
                .on_submit(DeckMessage::FinishEditName)
                .size(25)
                .into()
        } else {
            text(self.name()).size(25).into()
        };

        let plus_button = action(
            icon_plus(30.0),
            Some(DeckMessage::NewCard(self.cards.len())),
        );
        let pencil_button = action(icon_pencil(30.0), Some(DeckMessage::EditName));

        let review_button = action_btn("REVIEW", theme::Button::Default, DeckMessage::Review);

        let controls = row![
            deck_name,
            horizontal_space(Length::Fill),
            row![plus_button, pencil_button],
            review_button
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let cards: Vec<Element<_>> = self
            .cards
            .iter()
            .enumerate()
            .map(|(i, card)| {
                card.edit_view()
                    .map(move |message| DeckMessage::CardMessage(i, message))
            })
            .collect();

        column![controls, column(cards).spacing(10)]
            .spacing(15)
            .into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub card: brainace_core::Card,
    #[serde(skip)]
    pub revealed: bool,
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
            card: brainace_core::Card::new(front, back),
            revealed: false,
        }
    }

    pub fn update(&mut self, message: CardMessage) {
        match message {
            CardMessage::Hide => self.revealed = false,
            CardMessage::Reveal => self.revealed = true,
            CardMessage::ToggleState => self.revealed = !self.revealed,
            CardMessage::FrontChanged(content) => {
                self.card.set_front(&content);
            }
            CardMessage::BackChanged(content) => {
                self.card.set_back(&content);
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<CardMessage> {
        let front = text(self.card.front()).size(25);
        let back = text(self.card.back()).size(25).style(theme::Text::Accent);

        let content: Element<_> = if self.revealed {
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
        } else {
            container(front).padding([15, 25]).into()
        };

        container(content)
            .width(Length::Fill)
            .center_x()
            .style(theme::Container::Bordered)
            .into()
    }

    pub fn edit_view(&self) -> Element<CardMessage> {
        let front = text(self.card.front()).size(25);
        let back = text(self.card.back()).size(25).style(theme::Text::Accent);

        let eye_button = if self.revealed {
            icon_btn(icon_eye(20.0), Some(CardMessage::Hide))
        } else {
            icon_btn(icon_eye_off(20.0), Some(CardMessage::Reveal))
        };

        let edit_button = icon_btn(icon_pencil(20.0), Some(CardMessage::Edit));
        let trash_button = icon_btn(icon_trash(20.0), Some(CardMessage::Delete));

        let front_with_controls = row![
            front,
            horizontal_space(Length::Fill),
            eye_button.into(),
            edit_button.into(),
            trash_button.into()
        ]
        .spacing(5);

        let content: Element<_> = if self.revealed {
            let front_container = container(front_with_controls)
                .width(Length::Fill)
                .center_x()
                .padding([15, 25]);
            let back_container = container(back)
                .width(Length::Fill)
                .center_x()
                .padding([15, 25])
                .style(theme::Container::BorderedFooter);

            column![front_container, back_container].into()
        } else {
            container(front_with_controls).padding([15, 25]).into()
        };

        container(content)
            .width(Length::Fill)
            .center_x()
            .style(theme::Container::Bordered)
            .into()
    }
}
