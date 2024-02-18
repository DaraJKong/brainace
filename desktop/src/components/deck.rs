use brainace_core::{Card, Deck};
use iced::{
    widget::{column, component, horizontal_space, row, text, text_input, Component},
    Alignment, Length,
};

use crate::{
    action, action_btn, icon_pencil, icon_plus, theme,
    widget::{Element, Renderer},
};

use super::card::{card_edit_view, CardEvent};

pub struct DeckView<Message> {
    pub deck: Deck,
    editing_name: bool,
    on_review: Box<dyn Fn() -> Message + 'static>,
}

impl<Message> DeckView<Message> {
    fn new(deck: Deck, on_review: impl Fn() -> Message + 'static) -> Self {
        Self {
            deck,
            editing_name: false,
            on_review: Box::new(on_review),
        }
    }
}

pub fn deck_view<Message>(
    deck: Deck,
    on_review: impl Fn() -> Message + 'static,
) -> DeckView<Message> {
    DeckView::new(deck, on_review)
}

#[derive(Debug, Clone)]
pub enum DeckEvent {
    NewCard(usize),
    EditName,
    NameChanged(String),
    FinishEditName,
    Review,
    CardEvent(usize, CardEvent),
}

impl<Message> Component<Message, Renderer> for DeckView<Message> {
    type State = ();
    type Event = DeckEvent;

    fn update(&mut self, _state: &mut Self::State, event: DeckEvent) -> Option<Message> {
        match event {
            DeckEvent::NewCard(i) => {
                let card = Card::new("", "");
                self.deck.cards.push(card);

                None
            }
            DeckEvent::EditName => {
                self.editing_name = true;
                None
            }
            DeckEvent::NameChanged(content) => {
                self.deck.name = content;
                None
            }
            DeckEvent::FinishEditName => {
                self.editing_name = false;
                None
            }
            DeckEvent::Review => Some((self.on_review)()),
            DeckEvent::CardEvent(i, card_event) => match card_event {
                CardEvent::Delete => {
                    self.deck.cards.remove(i);
                    None
                }
                _ => None,
            },
        }
    }

    fn view(&self, _state: &Self::State) -> Element<DeckEvent> {
        let deck_name: Element<_> = if self.editing_name {
            text_input("Name the deck...", &self.deck.name)
                .on_input(DeckEvent::NameChanged)
                .on_submit(DeckEvent::FinishEditName)
                .size(25)
                .into()
        } else {
            text(self.deck.name.clone()).size(25).into()
        };

        let plus_button = action(
            icon_plus(30.0),
            Some(DeckEvent::NewCard(self.deck.cards.len())),
        );
        let pencil_button = action(icon_pencil(30.0), Some(DeckEvent::EditName));

        let review_button = action_btn("REVIEW", theme::Button::Default, DeckEvent::Review);

        let controls = row![
            deck_name,
            horizontal_space(Length::Fill),
            row![plus_button, pencil_button],
            review_button
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let cards: Vec<Element<_>> = self
            .deck
            .cards
            .iter()
            .enumerate()
            .map(|(i, card)| {
                card_edit_view(card.clone(), move |card_event| {
                    DeckEvent::CardEvent(i, card_event)
                })
                .into()
            })
            .collect();

        column![controls, column(cards).spacing(10)]
            .spacing(15)
            .into()
    }
}

impl<'a, Message: 'a> From<DeckView<Message>> for Element<'a, Message> {
    fn from(deck_view: DeckView<Message>) -> Self {
        component(deck_view)
    }
}
