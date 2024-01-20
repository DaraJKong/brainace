use brainace_core::Card;
use iced::{
    widget::{column, component, container, horizontal_space, row, text, Component},
    Length,
};

use crate::{
    icon_btn, icon_eye, icon_eye_off, icon_pencil, icon_trash, theme,
    widget::{Element, Renderer},
};

pub struct CardView {
    pub card: Card,
}

impl CardView {
    fn new(card: Card) -> Self {
        Self { card }
    }
}

pub fn card_view(card: Card) -> CardView {
    CardView::new(card)
}

impl<Message> Component<Message, Renderer> for CardView {
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<()> {
        let front = text(self.card.front()).size(25);
        let back = text(self.card.back()).size(25).style(theme::Text::Accent);

        let content: Element<_> = if self.card.revealed {
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
}

impl<'a, Message: 'a> From<CardView> for Element<'a, Message> {
    fn from(card_view: CardView) -> Self {
        component(card_view)
    }
}

pub struct CardEditView<Message> {
    card: Card,
    on_event: Box<dyn Fn(CardEvent) -> Message + 'static>,
}

impl<Message> CardEditView<Message> {
    fn new(card: Card, on_event: impl Fn(CardEvent) -> Message + 'static) -> Self {
        Self {
            card,
            on_event: Box::new(on_event),
        }
    }
}

pub fn card_edit_view<Message>(
    card: Card,
    on_event: impl Fn(CardEvent) -> Message + 'static,
) -> CardEditView<Message> {
    CardEditView::new(card, on_event)
}

#[derive(Clone, Debug)]
pub enum CardEvent {
    Toggle,
    Edit,
    Delete,
}

impl<Message> Component<Message, Renderer> for CardEditView<Message> {
    type State = ();
    type Event = CardEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            CardEvent::Toggle => {
                self.card.revealed = !self.card.revealed;
                None
            }
            CardEvent::Edit | CardEvent::Delete => Some((self.on_event)(event)),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<CardEvent> {
        let front = text(self.card.front()).size(25);
        let back = text(self.card.back()).size(25).style(theme::Text::Accent);

        let eye_button = if self.card.revealed {
            icon_btn(icon_eye(20.0), Some(CardEvent::Toggle))
        } else {
            icon_btn(icon_eye_off(20.0), Some(CardEvent::Toggle))
        };

        let edit_button = icon_btn(icon_pencil(20.0), Some(CardEvent::Edit));
        let trash_button = icon_btn(icon_trash(20.0), Some(CardEvent::Delete));

        let front_with_controls = row![
            front,
            horizontal_space(Length::Fill),
            eye_button.into(),
            edit_button.into(),
            trash_button.into()
        ]
        .spacing(5);

        let content: Element<_> = if self.card.revealed {
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

impl<'a, Message: 'a> From<CardEditView<Message>> for Element<'a, Message> {
    fn from(card_edit_view: CardEditView<Message>) -> Self {
        component(card_edit_view)
    }
}
