use std::collections::HashMap;

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

pub trait ConvertHighlightCodes {
    fn convert_highlight_codes(self) -> String;
}

impl ConvertHighlightCodes for String {
    fn convert_highlight_codes(self) -> String {
        let mut code_map = HashMap::new();

        code_map.insert("[red]", "\x1b[31m");
        code_map.insert("[green]", "\x1b[32m");
        code_map.insert("[yellow]", "\x1b[33m");
        code_map.insert("[blue]", "\x1b[34m");
        code_map.insert("[magenta]", "\x1b[35m");
        code_map.insert("[cyan]", "\x1b[36m");
        code_map.insert("[white]", "\x1b[37m");

        code_map.insert("[bold]", "\x1b[1m");
        code_map.insert("[italic]", "\x1b[3m");
        code_map.insert("[underline]", "\x1b[4m");
        code_map.insert("[reset]", "\x1b[0m");

        let mut result = self;

        for (replacement, code) in code_map {
            result = result.replace(replacement, code);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{ConvertEscapeCodes, ConvertHighlightCodes};

    #[test]
    fn test_escape_code_converter() {
        let input = "\x1b[31mHello \x1b[1mWorld\x1b[0m!".to_string();
        let expected = "[red]Hello [bold]World[reset]!";

        let actual = input.convert_escape_codes();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_code_converter() {
        let input = "[red]Hello [bold]World[reset]!".to_string();
        let expected = "\x1b[31mHello \x1b[1mWorld\x1b[0m!";

        let actual = input.convert_highlight_codes();

        assert_eq!(actual, expected);
    }
}
