use crate::highlighter::Highlight;
use crate::JsonConfig;
use nu_ansi_term::Style as NuStyle;
use serde_json::Value;

pub struct JsonHighlighter {
    pub key: NuStyle,
    pub curly_bracket: NuStyle,
    pub square_bracket: NuStyle,
    pub comma: NuStyle,
    pub colon: NuStyle,
}

impl JsonHighlighter {
    pub fn new(config: JsonConfig) -> Self {
        Self {
            key: config.key.into(),
            curly_bracket: config.curly_bracket.into(),
            square_bracket: config.square_bracket.into(),
            comma: config.comma.into(),
            colon: config.colon.into(),
        }
    }

    fn highlight_json(&self, value: Value) -> String {
        match value {
            Value::Object(map) => {
                let mut result = String::new();
                result.push_str(&self.curly_bracket.paint("{").to_string());

                for (i, (k, v)) in map.iter().enumerate() {
                    // Highlight the key
                    result.push_str(&self.key.paint(format!("\"{}\"", k)).to_string());
                    result.push_str(&self.colon.paint(": ").to_string());
                    // For now, leave the value unhighlighted
                    result.push_str(&v.to_string());

                    // Add a comma after each pair, except the last one
                    if i < map.len() - 1 {
                        result.push_str(&self.comma.paint(", ").to_string());
                    }
                }

                result.push_str(&self.curly_bracket.paint("}").to_string());
                result
            }
            Value::Array(arr) => {
                let mut result = String::new();
                result.push_str(&self.square_bracket.paint("[").to_string());

                for (i, v) in arr.iter().enumerate() {
                    result.push_str(&v.to_string());

                    // Add a comma between array values, except the last one
                    if i < arr.len() - 1 {
                        result.push_str(&self.comma.paint(", ").to_string());
                    }
                }

                result.push_str(&self.square_bracket.paint("]").to_string());
                result
            }
            // If it's any other type, return it as-is (values not highlighted)
            _ => value.to_string(),
        }
    }
}

impl Highlight for JsonHighlighter {
    fn apply(&self, input: &str) -> String {
        // First, attempt to parse the JSON
        let parsed_json: Value = match serde_json::from_str(input) {
            Ok(json) => json,
            Err(_) => return input.to_string(), // Return as-is if not valid JSON
        };

        // Convert the parsed JSON back into a string with highlighted elements
        self.highlight_json(parsed_json)
    }
}

#[cfg(test)]
mod tests {
    use crate::highlighter::Highlight;
    use crate::style::*;
    use crate::tests::escape_code_converter::ConvertEscapeCodes;

    use super::*;

    #[test]
    fn test_number_highlighter() {
        let config = JsonConfig {
            key: red(),
            quote_token: magenta(),
            curly_bracket: green(),
            square_bracket: yellow(),
            comma: cyan(),
            colon: blue(),
        };
        let highlighter = JsonHighlighter::new(config);

        let cases = vec![
            (r#"{ "name": "John", "age": 30 }"#, r#"{ "name": "John", "age": 30 }"#),
            (r#"{ "name": "John", "age": 30 }"#, r#"{ "name": "John", "age": 30 }"#),
            ("No jsons here!", "No jsons here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            println!("Actual: {}", actual);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
