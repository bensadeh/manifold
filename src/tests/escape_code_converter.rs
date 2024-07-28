use std::collections::HashMap;

#[derive(Debug)]
pub struct EscapeCodeConverter {
    code_map: HashMap<&'static str, &'static str>,
}

impl EscapeCodeConverter {
    pub fn new() -> Self {
        let mut code_map = HashMap::new();

        // Add mappings for colors
        code_map.insert("\x1b[31m", "[red]");
        code_map.insert("\x1b[32m", "[green]");
        code_map.insert("\x1b[33m", "[yellow]");
        code_map.insert("\x1b[34m", "[blue]");
        code_map.insert("\x1b[35m", "[magenta]");
        code_map.insert("\x1b[36m", "[cyan]");
        code_map.insert("\x1b[37m", "[white]");

        // Add mappings for text styles
        code_map.insert("\x1b[1m", "[bold]");
        code_map.insert("\x1b[3m", "[italic]");
        code_map.insert("\x1b[4m", "[underline]");
        code_map.insert("\x1b[0m", "[reset]");

        EscapeCodeConverter { code_map }
    }

    pub fn convert(&self, text: String) -> String {
        let mut result = text;

        for (code, replacement) in &self.code_map {
            result = result.replace(code, replacement);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_code_converter() {
        let converter = EscapeCodeConverter::new();
        let raw_text = "\x1b[31mHello \x1b[1mWorld\x1b[0m!".to_string();

        let converted_text = converter.convert(raw_text);

        assert_eq!(converted_text, "[red]Hello [bold]World[reset]!");
    }
}
