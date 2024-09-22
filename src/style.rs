use nu_ansi_term::{Color as NuColor, Style as NuStyle};
use serde::Deserialize;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy, Default, Deserialize)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub faint: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl From<&Color> for NuColor {
    fn from(color: &Color) -> Self {
        match color {
            Color::Default => NuColor::Default,
            Color::Black => NuColor::Black,
            Color::Red => NuColor::Red,
            Color::Green => NuColor::Green,
            Color::Yellow => NuColor::Yellow,
            Color::Blue => NuColor::Blue,
            Color::Magenta => NuColor::Magenta,
            Color::Cyan => NuColor::Cyan,
            Color::White => NuColor::White,
            Color::BrightBlack => NuColor::DarkGray,
            Color::BrightRed => NuColor::LightRed,
            Color::BrightGreen => NuColor::LightGreen,
            Color::BrightYellow => NuColor::LightYellow,
            Color::BrightBlue => NuColor::LightBlue,
            Color::BrightMagenta => NuColor::LightMagenta,
            Color::BrightCyan => NuColor::LightCyan,
            Color::BrightWhite => NuColor::LightGray,
        }
    }
}

impl From<Style> for NuStyle {
    fn from(style: Style) -> Self {
        let mut nu_style = NuStyle::new();

        if let Some(fg) = &style.fg {
            nu_style = nu_style.fg(NuColor::from(fg));
        }
        if let Some(bg) = &style.bg {
            nu_style = nu_style.on(NuColor::from(bg));
        }

        if style.bold {
            nu_style = nu_style.bold();
        }
        if style.faint {
            nu_style = nu_style.dimmed();
        }
        if style.italic {
            nu_style = nu_style.italic();
        }
        if style.underline {
            nu_style = nu_style.underline();
        }

        nu_style
    }
}

pub(crate) fn yellow() -> Style {
    Style {
        fg: Some(Color::Yellow),
        ..Style::default()
    }
}

pub(crate) fn green() -> Style {
    Style {
        fg: Some(Color::Green),
        ..Style::default()
    }
}

pub(crate) fn green_and_faint() -> Style {
    Style {
        fg: Some(Color::Green),
        faint: true,
        ..Style::default()
    }
}

pub(crate) fn cyan() -> Style {
    Style {
        fg: Some(Color::Cyan),
        ..Style::default()
    }
}

pub(crate) fn red() -> Style {
    Style {
        fg: Some(Color::Red),
        ..Style::default()
    }
}

pub(crate) fn red_and_faint() -> Style {
    Style {
        fg: Some(Color::Red),
        faint: true,
        ..Style::default()
    }
}

pub(crate) fn white() -> Style {
    Style {
        fg: Some(Color::White),
        ..Style::default()
    }
}

pub(crate) fn faint() -> Style {
    Style {
        faint: true,
        ..Style::default()
    }
}

pub(crate) fn blue() -> Style {
    Style {
        fg: Some(Color::Blue),
        ..Style::default()
    }
}

pub(crate) fn blue_and_faint() -> Style {
    Style {
        fg: Some(Color::Blue),
        faint: true,
        ..Style::default()
    }
}

pub(crate) fn blue_and_italic() -> Style {
    Style {
        fg: Some(Color::Blue),
        italic: true,
        ..Style::default()
    }
}

#[allow(dead_code)]
pub(crate) fn red_background() -> Style {
    Style {
        bg: Some(Color::Red),
        ..Style::default()
    }
}

pub(crate) fn magenta() -> Style {
    Style {
        fg: Some(Color::Magenta),
        ..Style::default()
    }
}

pub(crate) fn magenta_and_italic() -> Style {
    Style {
        fg: Some(Color::Magenta),
        italic: true,
        ..Style::default()
    }
}
