mod components;
mod reviewing;
mod theme;
mod widget;

use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use components::card_editor;
use iced::font::Weight;
use reviewing::{Card, CardMessage, Deck, DeckMessage};
use theme::palette::{CYAN_500, GREEN_500, ROSE_500, YELLOW_500};
use theme::Theme;
use widget::Element;

use chrono::Utc;

use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, progress_bar, row, text,
    text_editor,
};
use iced::{executor, window, Alignment, Application, Color, Command, Font, Length};
use widget::modal::Modal;

pub struct App {
    show_modal: bool,
    mode: Mode,
    fsrs: brainace_core::Config,
    deck: Option<Deck>,
    editing_id: usize,
    reviewing_id: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    ShowModal,
    HideModal,
    ChangeMode(Mode),
    Open,
    Save,
    CancelEdit,
    ConfirmEdit(String, String),
    DeckLoaded(Result<Arc<String>, Error>),
    DeckSaved(Result<PathBuf, Error>),
    DeckMessage(DeckMessage),
    CardMessage(usize, CardMessage),
    Skip,
    Reveal,
    Rate(u32),
    Continue,
    Close,
    Settings,
}

#[derive(Debug, Clone)]
enum Mode {
    Managing,
    Reviewing,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let fsrs = brainace_core::Config::default();

        let batch = [window::change_mode(
            window::Id::MAIN,
            window::Mode::Fullscreen,
        )];

        (
            Self {
                show_modal: false,
                mode: Mode::Managing,
                fsrs,
                deck: None,
                editing_id: 0,
                reviewing_id: 0,
            },
            Command::batch(batch),
        )
    }

    fn title(&self) -> String {
        String::from("Brainace")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ShowModal => {
                self.show_modal = true;
                Command::none()
            }
            Message::HideModal => {
                self.show_modal = false;
                Command::none()
            }
            Message::ChangeMode(mode) => {
                match mode {
                    Mode::Managing => {
                        self.mode = mode;
                    }
                    Mode::Reviewing => {
                        if let Some(deck) = &mut self.deck {
                            for card in &mut deck.cards {
                                card.update(CardMessage::Hide);
                            }

                            self.reviewing_id = 0;

                            self.mode = mode;
                        }
                    }
                }

                Command::none()
            }
            Message::Open => Command::perform(pick_file(), Message::DeckLoaded),
            Message::Save => {
                let text = ron::to_string(&self.deck).unwrap();

                Command::perform(
                    save_file(Some("mathematical_constants.ron".into()), text),
                    Message::DeckSaved,
                )
            }
            Message::CancelEdit => {
                self.show_modal = false;
                Command::none()
            }
            Message::ConfirmEdit(front, back) => {
                if let Some(deck) = &mut self.deck {
                    deck.cards[self.editing_id].card.set_front(&front);
                    deck.cards[self.editing_id].card.set_back(&back);
                }

                self.show_modal = false;
                Command::none()
            }
            Message::DeckLoaded(Ok(content)) => {
                self.deck = ron::from_str(content.as_str()).unwrap();

                Command::none()
            }
            Message::DeckSaved(Ok(path)) => Command::none(),
            Message::DeckLoaded(Err(error)) | Message::DeckSaved(Err(error)) => Command::none(),
            Message::DeckMessage(deck_message) => {
                if let Some(deck) = &mut self.deck {
                    match deck_message {
                        DeckMessage::CardMessage(i, CardMessage::Edit)
                        | DeckMessage::NewCard(i) => {
                            deck.update(deck_message);
                            self.editing_id = i;

                            self.show_modal = true;
                            Command::none()
                        }
                        DeckMessage::Review => self.update(Message::ChangeMode(Mode::Reviewing)),
                        _ => {
                            deck.update(deck_message);

                            Command::none()
                        }
                    }
                } else {
                    Command::none()
                }
            }
            Message::CardMessage(i, card_message) => {
                if let Some(deck) = &mut self.deck {
                    deck.cards[i].update(card_message);
                }

                Command::none()
            }
            Message::Skip => {
                if let Some(deck) = &self.deck {
                    self.reviewing_id = (self.reviewing_id + 1).clamp(0, deck.cards.len());
                }

                Command::none()
            }
            Message::Reveal => {
                if let Some(deck) = &mut self.deck {
                    if let Some(card) = deck.cards.get_mut(self.reviewing_id) {
                        if card.revealed {
                            self.reviewing_id = (self.reviewing_id + 1).clamp(0, deck.cards.len());
                        } else {
                            card.update(CardMessage::Reveal);
                        }
                    }
                }

                Command::none()
            }
            Message::Rate(rating) => {
                if let Some(deck) = &mut self.deck {
                    deck.cards[self.reviewing_id]
                        .card
                        .review(rating, Utc::now());

                    self.reviewing_id = (self.reviewing_id + 1).clamp(0, deck.cards.len());
                }

                Command::none()
            }
            Message::Continue | Message::Settings => Command::none(),
            Message::Close => window::close(window::Id::MAIN),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let page = match self.mode {
            Mode::Managing => manage_page(self.deck.as_ref()),
            Mode::Reviewing => review_page(self.deck.as_ref().unwrap(), self.reviewing_id),
        };

        let modal: Element<_> = self.deck.as_ref().map_or_else(
            || "".into(),
            |deck| {
                card_editor(
                    Some(deck.cards[self.editing_id].card.clone()),
                    || Message::CancelEdit,
                    |front, back| Message::ConfirmEdit(front.to_string(), back.to_string()),
                )
                .into()
            },
        );

        if self.show_modal {
            Modal::new(page, modal).on_blur(Message::HideModal).into()
        } else {
            page
        }
    }
}

fn manage_page(maybe_deck: Option<&Deck>) -> Element<'_, Message> {
    let (deck_info, deck_cards): (_, Element<_>) = maybe_deck.map_or_else(
        || {
            (
                text("No deck selected")
                    .size(25)
                    .style(theme::Text::Secondary),
                "".into(),
            )
        },
        |deck| {
            (
                text("Deck loaded").size(25).style(theme::Text::Secondary),
                deck.view().map(Message::DeckMessage),
            )
        },
    );

    let open_button = action_btn("OPEN", theme::Button::Default, Message::Open);

    let header = container(
        row![open_button, horizontal_space(Length::Fill), deck_info]
            .align_items(Alignment::Center)
            .spacing(15),
    )
    .width(1000)
    .height(150)
    .center_y();

    let main = container(deck_cards)
        .width(1000)
        .height(Length::Fill)
        .padding([25, 0]);

    column![header, horizontal_rule(0), main]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .into()
}

fn review_page(deck: &'_ Deck, id: usize) -> Element<'_, Message> {
    let cancel_icon = action(icon_cancel(25.0), Some(Message::Close));
    let cog_icon = action(icon_cog(25.0), Some(Message::Settings));

    let progress_bar = progress_bar(0.0..=(deck.cards.len() as f32), id as f32).height(15);

    let mut nunito_bold = Font::with_name("nunito");
    nunito_bold.weight = Weight::Semibold;

    let progress = text(format!("{} / {}", id, deck.cards.len()))
        .font(nunito_bold)
        .size(20)
        .style(theme::Text::Secondary);

    let header = container(
        row![cancel_icon, cog_icon, progress_bar, progress]
            .align_items(Alignment::Center)
            .spacing(15),
    )
    .width(1000)
    .height(150)
    .center_y();

    let main_content: Element<_> = if id == deck.cards.len() {
        container(text("Congratulations!").size(50))
            .width(Length::Fill)
            .center_x()
            .into()
    } else {
        deck.cards[id]
            .view()
            .map(move |message| Message::CardMessage(id, message))
    };

    let main = container(main_content)
        .width(1000)
        .height(Length::Fill)
        .center_y()
        .padding([0, 125]);

    let footer_content: Element<_> = if id == deck.cards.len() {
        let continue_button = action_btn(
            "CONTINUE",
            theme::Button::Default,
            Message::ChangeMode(Mode::Managing),
        );

        continue_button
    } else if deck.cards[id].revealed {
        let again_button = border_action_btn("AGAIN", ROSE_500, Message::Rate(1));
        let hard_button = border_action_btn("HARD", YELLOW_500, Message::Rate(2));
        let good_button = border_action_btn("GOOD", CYAN_500, Message::Rate(3));
        let easy_button = border_action_btn("EASY", GREEN_500, Message::Rate(4));

        container(row![again_button, hard_button, good_button, easy_button].spacing(15)).into()
    } else {
        let skip_button = border_btn(bold_text("SKIP"), Message::Skip);
        let reveal_button = action_btn("REVEAL", theme::Button::Default, Message::Reveal);

        row![
            skip_button.into(),
            horizontal_space(Length::Fill),
            reveal_button
        ]
        .into()
    };

    let footer = container(footer_content)
        .width(1000)
        .height(150)
        .center_x()
        .center_y();

    column![header, main, horizontal_rule(0), footer]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .into()
}

fn action<'a, Msg: 'a + Clone>(
    content: Element<'a, Msg>,
    on_press: Option<Msg>,
) -> Element<'a, Msg> {
    button(content)
        .on_press_maybe(on_press)
        .style(theme::Button::Text)
        .into()
}

fn icon_cancel<'a, Msg: 'a>(size: f32) -> Element<'a, Msg> {
    icon('\u{E804}', size)
}

fn icon_cog<'a, Msg: 'a>(size: f32) -> Element<'a, Msg> {
    icon('\u{E806}', size)
}

fn icon_eye<'a, Msg: 'a>(size: f32) -> Element<'a, Msg> {
    icon('\u{E807}', size)
}

fn icon_eye_off<'a, Msg: 'a>(size: f32) -> Element<'a, Msg> {
    icon('\u{E808}', size)
}

fn icon_pencil<'a, Msg: 'a>(size: f32) -> Element<'a, Msg> {
    icon('\u{E813}', size)
}

fn icon_trash<'a, Msg: 'a>(size: f32) -> Element<'a, Msg> {
    icon('\u{E800}', size)
}

fn icon_plus<'a, Msg: 'a>(size: f32) -> Element<'a, Msg> {
    icon('\u{E805}', size)
}

fn icon<'a, Msg: 'a>(codepoint: char, size: f32) -> Element<'a, Msg> {
    const ICON_FONT: Font = Font::with_name("app-icons");

    container(
        text(codepoint)
            .font(ICON_FONT)
            .size(size)
            .style(theme::Text::Secondary),
    )
    .height(Length::Fixed(size * 1.3))
    .width(Length::Fixed(size * 1.3))
    .center_x()
    .center_y()
    .into()
}

fn bold_text<Msg>(content: &str) -> Element<Msg> {
    let mut nunito_bold = Font::with_name("nunito");
    nunito_bold.weight = Weight::Semibold;

    text(content).font(nunito_bold).size(25).into()
}

fn border_btn<'a, Msg: 'a + Clone>(
    content: Element<'a, Msg>,
    on_press: Msg,
) -> impl Into<Element<'a, Msg>> {
    button(content)
        .on_press(on_press)
        .padding([10, 50])
        .style(theme::Button::Bordered)
}

fn icon_btn<'a, Msg: 'a + Clone>(
    content: Element<'a, Msg>,
    on_press_maybe: Option<Msg>,
) -> impl Into<Element<'a, Msg>> {
    button(content)
        .on_press_maybe(on_press_maybe)
        .style(theme::Button::Bordered)
}

fn border_action_btn<'a, Msg: 'a + Clone>(
    button_text: &str,
    color: Color,
    on_press: Msg,
) -> Element<'a, Msg> {
    let mut nunito_bold = Font::with_name("nunito");
    nunito_bold.weight = Weight::Semibold;

    button(text(button_text).font(nunito_bold).size(25))
        .on_press(on_press)
        .padding([10, 50])
        .style(theme::Button::BorderedAction(color))
        .into()
}

fn action_btn<'a, Msg: 'a + Clone>(
    action_text: &str,
    style: theme::Button,
    on_press: Msg,
) -> Element<'a, Msg> {
    let mut nunito_bold = Font::with_name("nunito");
    nunito_bold.weight = Weight::Semibold;

    button(
        text(action_text)
            .font(nunito_bold)
            .style(theme::Text::Dark)
            .size(25),
    )
    .on_press(on_press)
    .padding([10, 50])
    .style(style)
    .into()
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IOFailed(io::ErrorKind),
}

async fn pick_file() -> Result<Arc<String>, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<Arc<String>, Error> {
    let content = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IOFailed)?;

    Ok(content)
}

async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("Choose a file name...")
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(|handle| handle.path().to_owned())?
    };

    tokio::fs::write(&path, text)
        .await
        .map_err(|error| Error::IOFailed(error.kind()))?;

    Ok(path)
}
