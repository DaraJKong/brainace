use std::fmt;

use crate::{
    action, action_btn, border_btn, icon_btn, icon_eye, icon_eye_off, icon_pencil, icon_plus,
    icon_trash, theme, widget::Element,
};
use chrono::Utc;
pub use fsrs::Card as FSRSCard;
use fsrs::{Rating, FSRS};
use iced::{
    font::Weight,
    widget::{column, container, horizontal_space, row, text, text_input},
    Alignment, Font, Length,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deck {
    name: String,
    pub cards: Vec<Card>,
    #[serde(skip)]
    editing_name: bool,
    #[serde(skip)]
    editing_id: usize,
    #[serde(skip)]
    front_content: String,
    #[serde(skip)]
    back_content: String,
}

#[derive(Clone, Debug)]
pub enum DeckMessage {
    NewCard,
    EditName,
    NameChanged(String),
    FinishEditName,
    CardMessage(usize, CardMessage),
    FrontChanged(String),
    BackChanged(String),
    CancelEdit,
    ConfirmEdit,
}

impl Deck {
    pub fn new(name: &str) -> Deck {
        Deck {
            name: name.to_string(),
            cards: Vec::new(),
            editing_name: false,
            editing_id: 0,
            front_content: String::new(),
            back_content: String::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn edit(&mut self, id: usize) {
        self.editing_id = id;
        self.front_content = self.cards[id].front();
        self.back_content = self.cards[id].back();
    }

    pub fn update(&mut self, message: DeckMessage) {
        match message {
            DeckMessage::NewCard => {
                let card = Card::new();
                self.cards.push(card);

                let last_id = self.cards.len() - 1;
                self.edit(last_id);
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
                CardMessage::Edit => {
                    self.edit(i);
                }
                CardMessage::Delete => {
                    self.cards.remove(i);
                }
                _ => {
                    self.cards[i].update(message);
                }
            },
            DeckMessage::FrontChanged(content) => {
                self.front_content = content;
            }
            DeckMessage::BackChanged(content) => {
                self.back_content = content;
            }
            DeckMessage::CancelEdit => {}
            DeckMessage::ConfirmEdit => {
                self.cards[self.editing_id].front = self.front_content.clone();
                self.cards[self.editing_id].back = self.back_content.clone();
            }
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

        let plus_button = action(icon_plus(35.0), Some(DeckMessage::NewCard));
        let pencil_button = action(icon_pencil(35.0), Some(DeckMessage::EditName));

        let controls = row![
            deck_name,
            horizontal_space(Length::Fill),
            plus_button,
            pencil_button
        ];

        let cards = self
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

    pub fn card_editor(&self) -> Element<DeckMessage> {
        let front_input = text_input("Front", &self.front_content)
            .on_input(DeckMessage::FrontChanged)
            .size(25);
        let back_input = text_input("Back", &self.back_content)
            .on_input(DeckMessage::BackChanged)
            .size(25);

        let mut nunito_bold = Font::with_name("nunito");
        nunito_bold.weight = Weight::Semibold;

        let cancel_text = text("CANCEL").font(nunito_bold).size(25);

        let cancel_button = border_btn(cancel_text.into(), DeckMessage::CancelEdit);
        let ok_button = action_btn("OK", theme::Button::Default, DeckMessage::ConfirmEdit);

        container(
            column![
                front_input,
                back_input,
                row![cancel_button.into(), ok_button].spacing(15)
            ]
            .align_items(Alignment::Center)
            .spacing(15)
            .padding(50),
        )
        .width(Length::Fixed(850.0))
        .style(theme::Container::Modal)
        .into()
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
    pub fn new() -> Self {
        Self {
            front: String::new(),
            back: String::new(),
            state: CardState::default(),
            fsrs: FSRSCard::new(),
        }
    }

    pub fn front(&self) -> String {
        self.front.clone()
    }

    pub fn back(&self) -> String {
        self.back.clone()
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

    pub fn edit_view(&self) -> Element<CardMessage> {
        let front = text(self.front.clone()).size(25);
        let back = text(self.back.clone()).size(25).style(theme::Text::Accent);

        let eye_button = match self.state {
            CardState::Hidden => icon_btn(icon_eye_off(20.0), Some(CardMessage::Reveal)),
            CardState::Revealed => icon_btn(icon_eye(20.0), Some(CardMessage::Hide)),
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

        let content: Element<_> = match self.state {
            CardState::Hidden => container(front_with_controls).padding([15, 25]).into(),
            CardState::Revealed => {
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
            }
        };

        container(content)
            .width(Length::Fill)
            .center_x()
            .style(theme::Container::Bordered)
            .into()
    }
}
