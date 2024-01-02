mod reviewing;
mod theme;
mod widget;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, io};

use iced::font::Weight;
use iced::window::Mode;
use reviewing::{Card, CardMessage, Deck};
use theme::palette::{
    CYAN_300, CYAN_400, CYAN_500, GREEN_300, GREEN_400, GREEN_500, ROSE_300, ROSE_400, ROSE_500,
    SKY_400, SKY_500, YELLOW_300, YELLOW_400, YELLOW_500,
};
use theme::Theme;
use widget::Element;

use chrono::Utc;
use fsrs::{Rating, FSRS};

use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, progress_bar, row, text,
    text_editor,
};
use iced::{executor, window, Alignment, Application, Color, Command, Font, Length};

pub struct App {
    fsrs: FSRS,
    deck: Option<Deck>,
    reviewing_id: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Save,
    DeckLoaded(Result<Arc<String>, Error>),
    DeckSaved(Result<PathBuf, Error>),
    CardMessage(usize, CardMessage),
    Skip,
    Reveal,
    Rate(Rating),
    Continue,
    Close,
    Settings,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let fsrs = FSRS::default();

        let batch = [
            window::change_mode(window::Id::MAIN, Mode::Fullscreen),
            Command::perform(
                load_file("mathematical_constants.ron".into()),
                Message::DeckLoaded,
            ),
        ];

        (
            Self {
                fsrs,
                deck: None,
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
            Message::Load => Command::perform(
                load_file("mathematical_constants.ron".into()),
                Message::DeckLoaded,
            ),
            Message::Save => {
                let text = ron::to_string(&self.deck).unwrap();

                Command::perform(
                    save_file(Some("mathematical_constants.ron".into()), text),
                    Message::DeckSaved,
                )
            }
            Message::DeckLoaded(Ok(content)) => {
                self.deck = ron::from_str(content.as_str()).unwrap();

                Command::none()
            }
            Message::DeckSaved(Ok(path)) => Command::none(),
            Message::DeckLoaded(Err(error)) | Message::DeckSaved(Err(error)) => Command::none(),
            Message::CardMessage(i, CardMessage::Delete) => {
                if let Some(deck) = &mut self.deck {
                    deck.cards.remove(i);
                }

                Command::none()
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
                        if card.revealed() {
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
                    deck.cards[self.reviewing_id].schedule(self.fsrs, rating);
                    println!("{:?}", deck.cards[self.reviewing_id].log());

                    self.reviewing_id = (self.reviewing_id + 1).clamp(0, deck.cards.len());
                }

                Command::none()
            }
            Message::Continue => {
                if let Some(deck) = &mut self.deck {
                    for card in &mut deck.cards {
                        card.update(CardMessage::Hide);
                    }

                    self.reviewing_id = 0;
                }

                Command::none()
            }
            Message::Settings => Command::none(),
            Message::Close => window::close(window::Id::MAIN),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        if let Some(deck) = &self.deck {
            let cancel_icon = action(icon_cancel(25.0), Some(Message::Close));
            let cog_icon = action(icon_cog(25.0), Some(Message::Settings));

            let progress_bar =
                progress_bar(0.0..=(deck.cards.len() as f32), self.reviewing_id as f32).height(15);

            let mut nunito_bold = Font::with_name("nunito");
            nunito_bold.weight = Weight::Semibold;

            let progress = text(format!("{} / {}", self.reviewing_id, deck.cards.len()))
                .font(nunito_bold)
                .size(20)
                .style(theme::Text::Secondary);

            let header = container(
                row![cancel_icon, cog_icon, progress_bar, progress]
                    .align_items(Alignment::Center)
                    .spacing(15),
            )
            .width(Length::Fixed(1000.0))
            .height(150)
            .center_y();

            let main_content: Element<_> = if self.reviewing_id == deck.cards.len() {
                container(text("Congratulations!").size(50))
                    .width(Length::Fill)
                    .center_x()
                    .into()
            } else {
                deck.cards[self.reviewing_id]
                    .view()
                    .map(|message| Message::CardMessage(self.reviewing_id, message))
            };

            let main = container(main_content)
                .width(Length::Fixed(1000.0))
                .height(Length::Fill)
                .center_y()
                .padding([0, 125]);

            let footer_content: Element<_> = if self.reviewing_id == deck.cards.len() {
                let continue_button =
                    action_btn("CONTINUE", theme::Button::Default, Message::Continue);

                continue_button
            } else if deck.cards[self.reviewing_id].revealed() {
                let again_button =
                    border_action_btn("AGAIN", ROSE_500, Message::Rate(Rating::Again));
                let hard_button =
                    border_action_btn("HARD", YELLOW_500, Message::Rate(Rating::Hard));
                let good_button = border_action_btn("GOOD", CYAN_500, Message::Rate(Rating::Good));
                let easy_button = border_action_btn("EASY", GREEN_500, Message::Rate(Rating::Easy));

                container(row![again_button, hard_button, good_button, easy_button].spacing(15))
                    .into()
            } else {
                let skip_button = border_btn("SKIP", Message::Skip);
                let reveal_button = action_btn("REVEAL", theme::Button::Default, Message::Reveal);

                row![skip_button, horizontal_space(Length::Fill), reveal_button].into()
            };

            let footer = container(footer_content)
                .width(Length::Fixed(1000.0))
                .height(150)
                .center_x()
                .center_y();

            column![header, main, horizontal_rule(0), footer]
                .align_items(Alignment::Center)
                .into()
        } else {
            text("Loading deck...").style(theme::Text::Secondary).into()
        }
    }
}

fn action<'a>(content: Element<'a, Message>, on_press: Option<Message>) -> Element<'a, Message> {
    button(content)
        .on_press_maybe(on_press)
        .style(theme::Button::Text)
        .into()
}

fn icon_cancel<'a>(size: f32) -> Element<'a, Message> {
    icon('\u{E804}', size)
}

fn icon_cog<'a>(size: f32) -> Element<'a, Message> {
    icon('\u{E806}', size)
}

fn icon<'a>(codepoint: char, size: f32) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("app-icons");

    text(codepoint)
        .font(ICON_FONT)
        .size(size)
        .style(theme::Text::Secondary)
        .into()
}

fn border_btn(button_text: &str, on_press: Message) -> Element<'_, Message> {
    let mut nunito_bold = Font::with_name("nunito");
    nunito_bold.weight = Weight::Semibold;

    button(text(button_text).font(nunito_bold).size(25))
        .on_press(on_press)
        .padding([10, 50])
        .style(theme::Button::Bordered)
        .into()
}

fn border_action_btn(button_text: &str, color: Color, on_press: Message) -> Element<'_, Message> {
    let mut nunito_bold = Font::with_name("nunito");
    nunito_bold.weight = Weight::Semibold;

    button(text(button_text).font(nunito_bold).size(25))
        .on_press(on_press)
        .padding([10, 50])
        .style(theme::Button::BorderedAction(color))
        .into()
}

fn action_btn(action_text: &str, style: theme::Button, on_press: Message) -> Element<'_, Message> {
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
enum Error {
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
