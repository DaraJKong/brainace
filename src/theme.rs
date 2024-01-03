pub mod palette;

use iced::{
    application,
    widget::{
        button, container, progress_bar,
        rule::{self, FillMode},
        text,
    },
    BorderRadius, Color,
};

use self::palette::{EMERALD_400, EMERALD_500, GRAY_200, GRAY_700, GRAY_800, GRAY_900};

#[derive(Clone)]
pub struct Theme {
    background: Color,
    text: Color,
    surface: Color,
    border: Color,
    accent: Color,
    hover: Color,
}

impl Theme {
    pub const DARK: Self = Self {
        background: GRAY_900,
        text: GRAY_200,
        surface: GRAY_800,
        border: GRAY_700,
        accent: EMERALD_500,
        hover: EMERALD_400,
    };
}

impl Default for Theme {
    fn default() -> Self {
        Self::DARK
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.background,
            text_color: self.text,
        }
    }
}

#[derive(Clone, Default)]
pub enum Text {
    #[default]
    Default,
    Secondary,
    Dark,
    Accent,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => text::Appearance {
                color: self.text.into(),
            },
            Text::Secondary => text::Appearance {
                color: self.border.into(),
            },
            Text::Dark => text::Appearance {
                color: self.background.into(),
            },
            Text::Accent => text::Appearance {
                color: self.accent.into(),
            },
        }
    }
}

#[derive(Default)]
pub enum Container {
    #[default]
    Default,
    Surface,
    Bordered,
    BorderedFooter,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::Surface => container::Appearance {
                background: Some(self.surface.into()),
                ..Default::default()
            },
            Container::Bordered => container::Appearance {
                text_color: self.text.into(),
                border_radius: 10.0.into(),
                border_width: 2.0,
                border_color: self.surface,
                ..Default::default()
            },
            Container::BorderedFooter => container::Appearance {
                text_color: self.text.into(),
                background: Some(self.surface.into()),
                border_radius: [0.0, 0.0, 10.0, 10.0].into(),
                border_width: 2.0,
                border_color: self.surface,
            },
        }
    }
}

#[derive(Default)]
pub enum Button {
    #[default]
    Default,
    Bordered,
    Action((Color, Color)),
    BorderedAction(Color),
    Text,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance {
                background: Some(self.accent.into()),
                border_radius: 10.0.into(),
                text_color: self.background,
                ..Default::default()
            },
            Button::Action((color, _hover)) => button::Appearance {
                background: Some((*color).into()),
                border_radius: 10.0.into(),
                text_color: self.background,
                ..Default::default()
            },
            Button::Bordered | Button::BorderedAction(_) => button::Appearance {
                background: None,
                border_radius: 10.0.into(),
                border_width: 2.0,
                border_color: self.border,
                text_color: self.text,
                ..Default::default()
            },
            Button::Text => button::Appearance {
                background: None,
                text_color: self.border,
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        match style {
            Button::Default => button::Appearance {
                background: Some(self.hover.into()),
                ..active
            },
            Button::Action((_color, hover)) => button::Appearance {
                background: Some((*hover).into()),
                ..active
            },
            Button::Bordered => button::Appearance {
                background: Some(self.surface.into()),
                border_color: self.accent,
                text_color: self.accent,
                ..active
            },
            Button::BorderedAction(hover) => button::Appearance {
                background: Some(self.surface.into()),
                border_color: *hover,
                text_color: *hover,
                ..active
            },
            Button::Text => button::Appearance {
                text_color: self.text,
                ..active
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        match style {
            Button::Default | Button::Action(_) => button::Appearance {
                background: Some(self.surface.into()),
                ..active
            },
            Button::Bordered | Button::BorderedAction(_) => button::Appearance {
                border_color: self.surface,
                text_color: self.surface,
                ..active
            },
            Button::Text => button::Appearance {
                text_color: self.surface,
                ..active
            },
        }
    }
}

impl progress_bar::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> progress_bar::Appearance {
        progress_bar::Appearance {
            background: self.surface.into(),
            bar: self.accent.into(),
            border_radius: f32::MAX.into(),
        }
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: self.border,
            width: 2,
            radius: BorderRadius::default(),
            fill_mode: FillMode::Full,
        }
    }
}
