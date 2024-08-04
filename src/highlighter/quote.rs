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
    use crate::style::{red, yellow};

    use super::*;

    #[test]
    fn highlight_quotes_with_ansi() {
        let highlighter = QuoteHighlighter::new('"', yellow());

        let result = highlighter.apply("outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside");
        let expected = "outside \x1b[33m\"hello \x1b[34;42;3m42\x1b[0m\x1b[33m world\"\x1b[0m outside";

        assert_eq!(result, expected);
    }

    #[test]
    fn highlight_quotes_without_ansi() {
        let highlighter = QuoteHighlighter::new('"', red());

        let input = "outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside";
        let result = highlighter.apply(input);
        let expected = "outside \x1b[31m\"hello \x1b[34;42;3m42\x1b[0m\x1b[31m world\"\x1b[0m outside";

        assert_eq!(result, expected);
    }
}
