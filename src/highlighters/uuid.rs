use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

use crate::highlighter::Highlight;
use crate::UuidConfig;

pub struct UuidHighlighter {
    regex: Regex,
    number: NuStyle,
    letter: NuStyle,
    dash: NuStyle,
}

impl UuidHighlighter {
    pub fn new(config: UuidConfig) -> Result<Self, Error> {
        const UUID_REGEX: &str = r"(?x)       # Enable comments and whitespace insensitivity
            \b[0-9a-fA-F]{8}\b    # Match first segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match second segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match third segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match fourth segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{12}\b   # Match last segment of UUID
            ";

        let regex = Regex::new(UUID_REGEX)?;

        Ok(Self {
            regex,
            number: config.number.into(),
            letter: config.letter.into(),
            dash: config.dash.into(),
        })
    }
}

impl Highlight for UuidHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| {
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

#[cfg(test)]
mod tests {
    use crate::highlighter::Highlight;
    use crate::style::{blue, green, red};
    use crate::tests::escape_code_converter::ConvertEscapeCodes;

    use super::*;

    #[test]
    fn test_uuid_highlighter() {
        let highlighter = UuidHighlighter::new(UuidConfig {
            number: blue(),
            letter: green(),
            dash: red(),
        })
        .unwrap();

        let cases = vec![
            (
                "The UUID is 123e4567-e89b-12d3-a456-426614174000.",
                "The UUID is [blue]1[reset][blue]2[reset][blue]3[reset][green]e[reset][blue]4[reset][blue]5[reset][blue]6[reset][blue]7[reset][red]-[reset][green]e[reset][blue]8[reset][blue]9[reset][green]b[reset][red]-[reset][blue]1[reset][blue]2[reset][green]d[reset][blue]3[reset][red]-[reset][green]a[reset][blue]4[reset][blue]5[reset][blue]6[reset][red]-[reset][blue]4[reset][blue]2[reset][blue]6[reset][blue]6[reset][blue]1[reset][blue]4[reset][blue]1[reset][blue]7[reset][blue]4[reset][blue]0[reset][blue]0[reset][blue]0[reset]."
            ),
            (
                "Another UUID is f47ac10b-58cc-4372-a567-0e02b2c3d479.",
                "Another UUID is [green]f[reset][blue]4[reset][blue]7[reset][green]a[reset][green]c[reset][blue]1[reset][blue]0[reset][green]b[reset][red]-[reset][blue]5[reset][blue]8[reset][green]c[reset][green]c[reset][red]-[reset][blue]4[reset][blue]3[reset][blue]7[reset][blue]2[reset][red]-[reset][green]a[reset][blue]5[reset][blue]6[reset][blue]7[reset][red]-[reset][blue]0[reset][green]e[reset][blue]0[reset][blue]2[reset][green]b[reset][blue]2[reset][green]c[reset][blue]3[reset][green]d[reset][blue]4[reset][blue]7[reset][blue]9[reset]."
            ),
            (
                "No UUID here!",
                "No UUID here!"
            ),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
