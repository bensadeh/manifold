use crate::highlighter::Highlight;
use crate::JsonConfig;
use nu_ansi_term::Style as NuStyle;
use serde_json::Value;
use std::fmt::Write;

pub struct JsonHighlighter {
    pub key: NuStyle,
    pub quote_token: NuStyle,
    pub curly_bracket: NuStyle,
    pub square_bracket: NuStyle,
    pub comma: NuStyle,
    pub colon: NuStyle,
}

impl JsonHighlighter {
    pub fn new(config: JsonConfig) -> Self {
        Self {
            key: config.key.into(),
            quote_token: config.quote_token.into(),
            curly_bracket: config.curly_bracket.into(),
            square_bracket: config.square_bracket.into(),
            comma: config.comma.into(),
            colon: config.colon.into(),
        }
    }

    fn format_json(&self, value: &Value, output: &mut String) {
        match value {
            Value::Object(map) => {
                write!(output, "{}", self.curly_bracket.paint("{")).unwrap();
                let mut first = true;
                for (key, val) in map {
                    if !first {
                        write!(output, "{}", self.comma.paint(",")).unwrap();
                    }
                    first = false;

                    write!(
                        output,
                        " {}{}{} ",
                        self.quote_token.paint("\""),
                        self.key.paint(key),
                        self.quote_token.paint("\"")
                    )
                    .unwrap();
                    write!(output, "{} ", self.colon.paint(":")).unwrap();

                    // Recursively format the value
                    self.format_json(val, output);
                }
                write!(output, " {}", self.curly_bracket.paint("}")).unwrap();
            }
            Value::Array(array) => {
                write!(output, "{}", self.square_bracket.paint("[")).unwrap();
                let mut first = true;
                for item in array {
                    if !first {
                        write!(output, "{}", self.comma.paint(",")).unwrap();
                    }
                    first = false;

                    // Recursively format the array items
                    self.format_json(item, output);
                }
                write!(output, "{}", self.square_bracket.paint("]")).unwrap();
            }
            Value::String(s) => {
                write!(
                    output,
                    "{}{}{}",
                    self.quote_token.paint("\""),
                    s,
                    self.quote_token.paint("\"")
                )
                .unwrap();
            }
            Value::Number(n) => {
                write!(output, "{}", n).unwrap();
            }
            Value::Bool(b) => {
                write!(output, "{}", b).unwrap();
            }
            Value::Null => {
                write!(output, "null").unwrap();
            }
        }
    }
}

impl Highlight for JsonHighlighter {
    fn apply(&self, input: &str) -> String {
        // Attempt to parse the input as JSON
        let json_value: Value = match serde_json::from_str(input) {
            Ok(value) => value,
            Err(_) => return input.to_string(), // Return as-is if not valid JSON
        };

        let mut output = String::new();
        self.format_json(&json_value, &mut output);
        output
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
            key: yellow(),
            quote_token: blue(),
            curly_bracket: cyan(),
            square_bracket: green(),
            comma: red(),
            colon: magenta(),
        };
        let highlighter = JsonHighlighter::new(config);

        let cases = vec![
            (
                r#"{ "name": "John Doe", "age": 43, "phones": [ "+44 1234567", "+44 2345678" ] }"#,
                r#"{ "name": "John", "age": 30 }"#,
            ),
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
