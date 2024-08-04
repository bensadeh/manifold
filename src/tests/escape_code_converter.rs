use std::collections::HashMap;

use crate::style::Color::{Blue, Green, Red};
use crate::style::Style;

pub trait ConvertEscapeCodes {
    fn convert_escape_codes(self) -> String;
}

impl ConvertEscapeCodes for String {
    fn convert_escape_codes(self) -> String {
        let mut code_map = HashMap::new();

        code_map.insert("\x1b[31m", "[red]");
        code_map.insert("\x1b[32m", "[green]");
        code_map.insert("\x1b[33m", "[yellow]");
        code_map.insert("\x1b[34m", "[blue]");
        code_map.insert("\x1b[35m", "[magenta]");
        code_map.insert("\x1b[36m", "[cyan]");
        code_map.insert("\x1b[37m", "[white]");

        code_map.insert("\x1b[1m", "[bold]");
        code_map.insert("\x1b[3m", "[italic]");
        code_map.insert("\x1b[4m", "[underline]");
        code_map.insert("\x1b[0m", "[reset]");

        let mut result = self;

        for (code, replacement) in code_map {
            result = result.replace(code, replacement);
        }

        result
    }
}

pub fn red() -> Style {
    Style {
        fg: Some(Red),
        ..Style::default()
    }
}

pub fn blue() -> Style {
    Style {
        fg: Some(Blue),
        ..Style::default()
    }
}

pub fn green() -> Style {
    Style {
        fg: Some(Green),
        ..Style::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::escape_code_converter::ConvertEscapeCodes;

    #[test]
    fn test_escape_code_converter() {
        let input = "\x1b[31mHello \x1b[1mWorld\x1b[0m!".to_string();
        let expected = "[red]Hello [bold]World[reset]!";

        let actual = input.convert_escape_codes();

        assert_eq!(actual, expected);
    }
}
