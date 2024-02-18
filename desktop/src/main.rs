use iced::window::{Position, Settings as WindowSettings};
use iced::{Application, Font, Settings};

use brainace_desktop::App;

fn main() -> Result<(), iced::Error> {
    let nunito_font = Font::with_name("nunito");

    App::run(Settings {
        default_font: nunito_font,
        fonts: vec![
            include_bytes!("../fonts/Nunito-VariableFont_wght.ttf")
                .as_slice()
                .into(),
            include_bytes!("../fonts/app-icons.ttf").as_slice().into(),
        ],
        window: WindowSettings {
            position: Position::Centered,
            ..WindowSettings::default()
        },
        ..Settings::default()
    })
}
