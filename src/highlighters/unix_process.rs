use nu_ansi_term::Style as NuStyle;
use regex::{Error, Regex};

use crate::highlighter::Highlight;
use crate::UnixProcessConfig;

pub struct UnixProcessHighlighter {
    regex: Regex,
    name: NuStyle,
    id: NuStyle,
    bracket: NuStyle,
}

impl UnixProcessHighlighter {
    pub fn new(config: UnixProcessConfig) -> Result<Self, Error> {
        let regex = Regex::new(r"(?P<process_name>\([^)]+\)|[\w-]+)\[(?P<process_id>\d+)]")?;

        Ok(Self {
            regex,
            name: config.name.into(),
            id: config.id.into(),
            bracket: config.bracket.into(),
        })
    }
}

impl Highlight for UnixProcessHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |captures: &regex::Captures| {
                let process_name = captures
                    .name("process_name")
                    .map(|p| format!("{}", self.name.paint(p.as_str())))
                    .unwrap_or_default();
                let process_num = captures
                    .name("process_id")
                    .map(|n| format!("{}", self.id.paint(n.as_str())))
                    .unwrap_or_default();

                format!(
                    "{}{}{}{}",
                    process_name,
                    self.bracket.paint("["),
                    process_num,
                    self.bracket.paint("]")
                )
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
    fn test_unix_process_highlighter() {
        let highlighter = UnixProcessHighlighter::new(UnixProcessConfig {
            name: Style::new().fg(Color::Magenta),
            id: Style::new().fg(Color::Green),
            bracket: Style::new().fg(Color::Blue),
        })
        .unwrap();

        let cases = vec![
            (
                "process[1]",
                "[magenta]process[reset][blue][[reset][green]1[reset][blue]][reset]",
            ),
            ("No process here!", "No process here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
