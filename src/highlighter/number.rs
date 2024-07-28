use nu_ansi_term::Style as NuStyle;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::manifold::Highlight;
use crate::style::Style;

static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)       # Enable comments and whitespace insensitivity
            \b       # Word boundary, ensures we are at the start of a number
            \d+      # Matches one or more digits
            (\.      # Start a group to match a decimal part
            \d+      # Matches one or more digits after the dot
            )?       # The decimal part is optional
            \b       # Word boundary, ensures we are at the end of a number
            ",
    )
    .expect("Invalid regex pattern")
});

pub struct NumberHighlighter {
    style: NuStyle,
}

impl NumberHighlighter {
    pub fn new(style: Style) -> Self {
        Self {
            style: style.into(),
        }
    }
}

impl Highlight for NumberHighlighter {
    fn apply(&self, input: String) -> String {
        NUMBER_REGEX
            .replace_all(input.as_str(), |caps: &Captures<'_>| {
                format!("{}", self.style.paint(&caps[0]))
            })
            .to_string()
    }
}
