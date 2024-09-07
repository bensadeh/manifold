use crate::highlighter::Highlight;
use crate::DateTimeConfig;
use nu_ansi_term::Style as NuStyle;
use nu_ansi_term::Style;
use regex::{Error, Regex};

pub struct TimeHighlighter {
    regex: Regex,
    time: NuStyle,
    zone: NuStyle,
    separator: NuStyle,
}

impl TimeHighlighter {
    pub fn new(time_config: DateTimeConfig) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)
            \b                                         # Word boundary, ensures we are at the start of a time
            (?P<T>[T\s])?                              # Capture separator (either a space or T)
            (?P<hours>\d{1,2})(?P<colon1>:)
            (?P<minutes>\d{2})(?P<colon2>:)
            (?P<seconds>\d{2})
            (?P<frac_sep>[.,:])?(?P<frac_digits>\d+)?  # Capture fractional seconds (separator and digits separately)
            (?P<tz>Z)?            
            ",
        )?;

        Ok(Self {
            regex,
            time: time_config.time.into(),
            zone: time_config.zone.into(),
            separator: time_config.separator.into(),
        })
    }
}

impl Highlight for TimeHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &regex::Captures<'_>| {
                let paint_and_stringify = |name: &str, style: &Style| {
                    caps.name(name)
                        .map(|m| style.paint(m.as_str()).to_string())
                        .unwrap_or_default()
                };

                let parts = [
                    ("T", &self.zone),
                    ("hours", &self.time),
                    ("colon1", &self.separator),
                    ("minutes", &self.time),
                    ("colon2", &self.separator),
                    ("seconds", &self.time),
                    ("frac_sep", &self.separator),
                    ("frac_digits", &self.time),
                    ("tz", &self.zone),
                ];

                parts.iter().fold(String::new(), |acc, (name, style)| {
                    acc + &paint_and_stringify(name, style)
                })
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
    fn test_time_highlighter() {
        let config = DateTimeConfig {
            date: Default::default(),
            time: red(),
            zone: blue(),
            separator: yellow(),
        };
        let highlighter = TimeHighlighter::new(config).unwrap();

        let cases = vec![
            (
                "07:46:34",
                "[red]07[reset][yellow]:[reset][red]46[reset][yellow]:[reset][red]34[reset]",
            ),
            (
                "10:51:19.251",
                "[red]10[reset][yellow]:[reset][red]51[reset][yellow]:[reset][red]19[reset][yellow].[reset][red]251[reset]"
             ),
            (
                "11:47:39:850",
                "[red]11[reset][yellow]:[reset][red]47[reset][yellow]:[reset][red]39[reset][yellow]:[reset][red]850[reset]"
            ),
            (
                "3:33:30",
                "[red]3[reset][yellow]:[reset][red]33[reset][yellow]:[reset][red]30[reset]"
            ),
            (
                "2022-09-09 11:48:34,534",
                "2022-09-09[blue] [reset][red]11[reset][yellow]:[reset][red]48[reset][yellow]:[reset][red]34[reset][yellow],[reset][red]534[reset]"
            ),
            (
                "2022-09-22T07:46:34.171800155Z",
                "2022-09-22T07:46:34.171800155Z"
            ),
            ("No time here!", "No time here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
