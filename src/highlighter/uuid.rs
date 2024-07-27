use nu_ansi_term::Style as NuStyle;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::highlighter::Highlight;
use crate::style::Style;

static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
            \b[0-9a-fA-F]{8}\b    # Match first segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match second segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match third segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match fourth segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{12}\b   # Match last segment of UUID
        ",
    )
    .expect("Invalid UUID regex pattern")
});

pub struct UuidHighlighter {
    number: NuStyle,
    letter: NuStyle,
    dash: NuStyle,
}

impl UuidHighlighter {
    pub fn new(number_style: Style, letter_style: Style, dash_style: Style) -> Self {
        Self {
            number: number_style.into(),
            letter: letter_style.into(),
            dash: dash_style.into(),
        }
    }
}

impl Highlight for UuidHighlighter {
    fn apply(&self, input: String) -> String {
        UUID_REGEX
            .replace_all(input.as_str(), |caps: &Captures<'_>| {
                caps[0]
                    .chars()
                    .map(|c| match c {
                        '0'..='9' => format!("{}", self.number.paint(c.to_string())),
                        'a'..='f' | 'A'..='F' => format!("{}", self.letter.paint(c.to_string())),
                        '-' => format!("{}", self.dash.paint(c.to_string())),
                        _ => c.to_string(),
                    })
                    .collect::<String>()
            })
            .to_string()
    }
}
