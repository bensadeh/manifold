use crate::highlighter::Highlight;
use crate::DateTimeConfig;
use nu_ansi_term::Style as NuStyle;
use nu_ansi_term::Style;
use regex::{Error, Regex};

pub struct DateDashHighlighter {
    regex: Regex,
    time: NuStyle,
    zone: NuStyle,
    separator: NuStyle,
}

impl DateDashHighlighter {
    pub fn new(time_config: DateTimeConfig) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)
            (?P<year>19\d{2}|20\d{2})            // Year: 1900-2099
            (?P<separator1>-)                    // Separator (dash)
            (?:
                (?P<month>0[1-9]|1[0-2])         // Month: 01-12
                (?P<separator2>-)
                (?P<day>0[1-9]|[12]\d|3[01])     // Day: 01-31
                |
                (?P<day_alt>0[1-9]|[12]\d|3[01]) // Day: 01-31
                (?P<separator2_alt>-)
                (?P<month_alt>0[1-9]|1[0-2])     // Month: 01-12
            )
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

impl Highlight for DateDashHighlighter {
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
    use crate::tests::escape_code_converter::ConvertEscapeCodes;

    use super::*;

    #[test]
    fn test_time_highlighter() {
        let config = DateTimeConfig::default();
        let highlighter = DateDashHighlighter::new(config).unwrap();

        let cases = vec![
            (
                "07:46:34",
                "[red]07[reset][yellow]:[reset][red]46[reset][yellow]:[reset][red]34[reset]",
            ),
            ("No time here!", "No time here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
