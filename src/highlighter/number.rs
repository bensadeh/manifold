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
        Self { style: style.into() }
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

#[cfg(test)]
mod tests {
    use crate::manifold::Highlight;
    use crate::style::Color::Red;
    use crate::style::Style;
    use crate::tests::escape_code_converter::ConvertEscapeCodes;

    use super::*;

    #[test]
    fn test_number_highlighter() {
        let style = Style {
            fg: Some(Red),
            ..Style::default()
        };
        let highlighter = NumberHighlighter::new(style);

        let input = "The fox jumps over 13 dogs. The number 42.5 is here.".to_string();
        let expected = "The fox jumps over [red]13[reset] dogs. The number [red]42.5[reset] is here.".to_string();

        let actual = highlighter.apply(input);

        assert_eq!(expected, actual.convert_escape_codes());
    }
}
