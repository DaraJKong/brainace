use brainace_core::Card;
use iced::{
    font::Weight,
    widget::{column, component, container, row, text, text_input, Component},
    Alignment, Font, Length,
};

use crate::{
    action_btn, border_btn, theme,
    widget::{Element, Renderer},
};

pub struct CardEditor<Message> {
    front: String,
    back: String,
    on_cancel: Box<dyn Fn() -> Message + 'static>,
    on_confirm: Box<dyn Fn(&str, &str) -> Message + 'static>,
}

impl<Message> CardEditor<Message> {
    fn new(
        card: Option<Card>,
        on_cancel: impl Fn() -> Message + 'static,
        on_confirm: impl Fn(&str, &str) -> Message + 'static,
    ) -> Self {
        let (front, back) = match card {
            Some(card) => (card.front(), card.back()),
            None => (String::new(), String::new()),
        };

        Self {
            front,
            back,
            on_cancel: Box::new(on_cancel),
            on_confirm: Box::new(on_confirm),
        }
    }
}

pub fn card_editor<Message>(
    card: Option<Card>,
    on_cancel: impl Fn() -> Message + 'static,
    on_confirm: impl Fn(&str, &str) -> Message + 'static,
) -> CardEditor<Message> {
    CardEditor::new(card, on_cancel, on_confirm)
}

#[derive(Clone)]
pub enum Event {
    FrontChanged(String),
    BackChanged(String),
    Cancel,
    Confirm,
}

impl<Message> Component<Message, Renderer> for CardEditor<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::FrontChanged(content) => {
                self.front = content;
                None
            }
            Event::BackChanged(content) => {
                self.back = content;
                None
            }
            Event::Cancel => Some((self.on_cancel)()),
            Event::Confirm => Some((self.on_confirm)(&self.front, &self.back)),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Event> {
        let front_input = text_input("Front", &self.front)
            .on_input(Event::FrontChanged)
            .size(25);
        let back_input = text_input("Back", &self.back)
            .on_input(Event::BackChanged)
            .size(25);

        let mut nunito_bold = Font::with_name("nunito");
        nunito_bold.weight = Weight::Semibold;

        let cancel_text = text("CANCEL").font(nunito_bold).size(25);

        let cancel_button = border_btn(cancel_text.into(), Event::Cancel);
        let ok_button = action_btn("OK", theme::Button::Default, Event::Confirm);

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

impl<'a, Message: 'a> From<CardEditor<Message>> for Element<'a, Message> {
    fn from(card_editor: CardEditor<Message>) -> Self {
        component(card_editor)
    }
}
