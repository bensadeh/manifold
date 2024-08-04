use nu_ansi_term::Style as NuStyle;

use crate::highlighter::quote::State::{InsideQuote, OutsideQuote};
use crate::manifold::Highlight;
use crate::style::Style;

const RESET: &str = "\x1b[0m";

pub struct QuoteHighlighter {
    quotes_token: char,
    color: String,
}

impl QuoteHighlighter {
    pub fn new(quotes_token: char, style: Style) -> Self {
        Self {
            quotes_token,
            color: ansi_color_code_without_reset(style),
        }
    }
}

fn ansi_color_code_without_reset(style: Style) -> String {
    let nu_style = NuStyle::from(style);
    let styled_str = format!("{}", nu_style.paint(""));

    styled_str.replace(RESET, "")
}

impl Highlight for QuoteHighlighter {
    fn apply(&self, input: &str) -> String {
        let quotes_count = input.chars().filter(|&ch| ch == self.quotes_token).count();

        if quotes_count % 2 != 0 {
            return input.to_string();
        }

        let mut state = OutsideQuote;
        let mut output = String::new();

        for ch in input.chars() {
            match &mut state {
                InsideQuote {
                    color_inside_quote: color,
                    ref mut potential_reset_code,
                } => {
                    if ch == self.quotes_token {
                        output.push(ch);
                        output.push_str(RESET);
                        state = OutsideQuote;
                        continue;
                    }

                    potential_reset_code.push(ch);
                    if potential_reset_code.as_str() == RESET {
                        output.push_str(potential_reset_code);
                        output.push_str(color);
                        potential_reset_code.clear();
                    } else if !RESET.starts_with(potential_reset_code.as_str()) {
                        output.push_str(potential_reset_code);
                        potential_reset_code.clear();
                    }
                }
                OutsideQuote => {
                    if ch == self.quotes_token {
                        output.push_str(&self.color);
                        output.push(ch);
                        state = InsideQuote {
                            color_inside_quote: self.color.clone(),
                            potential_reset_code: String::new(),
                        };
                        continue;
                    }

                    output.push(ch);
                }
            };
        }

        output
    }
}

enum State {
    InsideQuote {
        color_inside_quote: String,
        potential_reset_code: String,
    },
    OutsideQuote,
}

#[cfg(test)]
mod tests {
    use crate::style::*;
    use crate::tests::escape_code_converter::{ConvertEscapeCodes, ConvertHighlightCodes};

    use super::*;

    #[test]
    fn test_multiple() {
        let highlighter = QuoteHighlighter::new('"', yellow());

        let cases = vec![
            (
                r#"Lorem ipsum "dolor" sit amet"#,
                r#"Lorem ipsum [yellow]"dolor"[reset] sit amet"#,
            ),
            (
                r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit"#,
                r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit"#,
            ),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }

    #[test]
    fn test_no_overwrite() {
        let highlighter = QuoteHighlighter::new('"', yellow());

        let input = r#"Hello "abc [red]def[reset] ghi" World"#.to_string().convert_highlight_codes();
        let expected = r#"Hello [yellow]"abc [red]def[reset][yellow] ghi"[reset] World"#;

        let actual = highlighter.apply(input.as_str());

        assert_eq!(actual.convert_escape_codes(), expected);
    }

    #[test]
    fn test_odd_number_of_highlight_tokens() {
        let highlighter = QuoteHighlighter::new('"', yellow());

        let input = r#"Hello "abc def ghi World"#;
        let expected = r#"Hello "abc def ghi World"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.convert_escape_codes(), expected);
    }
}
