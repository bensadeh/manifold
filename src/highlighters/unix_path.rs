use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

use crate::highlighter::Highlight;
use crate::UnixPathConfig;

pub struct UnixPathHighlighter {
    regex: Regex,
    segment: NuStyle,
    separator: NuStyle,
}

impl UnixPathHighlighter {
    pub fn new(config: UnixPathConfig) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)               # Enable comments and whitespace insensitivity
            (?P<path>            # Capture the path segment
                [~/.][\w./-]*    # Match zero or more word characters, dots, slashes, or hyphens
                /[\w.-]*         # Match a path segment separated by a slash
            )",
        )?;

        Ok(Self {
            regex,
            segment: config.segment.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for UnixPathHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| {
                let mut output = String::new();
                let path = &caps[0];
                let chars: Vec<_> = path.chars().collect();

                // Check if path starts with a valid character and not a double slash
                if !(chars[0] == '/' || chars[0] == '~' || (chars[0] == '.' && chars.len() > 1 && chars[1] == '/'))
                    || (chars[0] == '/' && chars.len() > 1 && chars[1] == '/')
                {
                    return path.to_string();
                }

                for &char in &chars {
                    match char {
                        '/' => output.push_str(&format!("{}", self.separator.paint(char.to_string()))),
                        _ => output.push_str(&format!("{}", self.segment.paint(char.to_string()))),
                    }
                }

                output
            })
            .to_string()
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
        let highlighter = UnixPathHighlighter::new(UnixPathConfig {
            segment: green(),
            separator: yellow(),
        })
        .unwrap();

        let cases = vec![
            ("/user/local",

             "[yellow]/[reset][green]u[reset][green]s[reset][green]e[reset][green]r[reset][yellow]/[reset][green]l[reset][green]o[reset][green]c[reset][green]a[reset][green]l[reset]"),
                         ("No numbers here!", "No numbers here!")];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
