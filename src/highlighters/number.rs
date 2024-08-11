use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

use crate::highlighter::Highlight;
use crate::NumberConfig;

pub struct NumberHighlighter {
    regex: Regex,
    style: NuStyle,
}

impl NumberHighlighter {
    pub fn new(config: NumberConfig) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)   # Enable comments and whitespace insensitivity
            \b       # Word boundary, ensures we are at the start of a number
            \d+      # Matches one or more digits
            (\.      # Start a group to match a decimal part
            \d+      # Matches one or more digits after the dot
            )?       # The decimal part is optional
            \b       # Word boundary, ensures we are at the end of a number
            ",
        )?;

        Ok(Self {
            regex,
            style: config.number.into(),
        })
    }
}

impl Highlight for NumberHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| format!("{}", self.style.paint(&caps[0])))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::highlighter::Highlight;
    use crate::style::red;
    use crate::tests::escape_code_converter::ConvertEscapeCodes;

    use super::*;

    #[test]
    fn test_number_highlighter() {
        let highlighter = NumberHighlighter::new(NumberConfig { number: red() }).unwrap();

        let cases = vec![
            (
                "The fox jumps over 13 dogs. The number 42.5 is here.",
                "The fox jumps over [red]13[reset] dogs. The number [red]42.5[reset] is here.",
            ),
            (
                "There are 1001 nights in the tale.",
                "There are [red]1001[reset] nights in the tale.",
            ),
            ("No numbers here!", "No numbers here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
