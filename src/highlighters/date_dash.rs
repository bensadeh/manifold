use crate::highlighter::Highlight;
use crate::DateTimeConfig;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

pub struct DateDashHighlighter {
    regex_yyyy_xx_xx: Regex,
    regex_xx_xx_yyyy: Regex,
    date: NuStyle,
    separator: NuStyle,
}

impl DateDashHighlighter {
    pub fn new(time_config: DateTimeConfig) -> Result<Self, Error> {
        let regex_yyyy_xx_xx = Regex::new(
            r"(?x)
                (?P<year>19\d{2}|20\d{2})            # Year: 1900-2099
                (?P<separator>[-/])                  # Separator (dash or slash)
                (?P<first>0[1-9]|[12]\d|3[01])       # First number: 01-31
                (?P<separator2>[-/])                 # Separator (dash or slash)
                (?P<second>0[1-9]|[12]\d|3[01])      # Second number: 01-31
                ",
        )?;

        let regex_xx_xx_yyyy = Regex::new(
            r"(?x)
                (?P<first>0[1-9]|[12]\d|3[01])       # First number: 01-31
                (?P<separator>[-/])                  # Separator (dash or slash)
                (?P<second>0[1-9]|[12]\d|3[01])      # Second number: 01-31
                (?P<separator2>[-/])                 # Separator (dash or slash)
                (?P<year>19\d{2}|20\d{2})            # Year: 1900-2099
                ",
        )?;

        Ok(Self {
            regex_yyyy_xx_xx,
            regex_xx_xx_yyyy,
            date: time_config.time.into(),
            separator: time_config.separator.into(),
        })
    }
}

impl Highlight for DateDashHighlighter {
    fn apply(&self, input: &str) -> String {
        let first_string = self
            .regex_yyyy_xx_xx
            .replace_all(input, |caps: &Captures<'_>| {
                let year = caps.name("year").map(|m| m.as_str());
                let month = caps.name("month").map(|m| m.as_str());
                let day = caps.name("day").map(|m| m.as_str());
                let separator1 = caps.name("separator1").map(|m| m.as_str());
                let separator2 = caps.name("separator2").map(|m| m.as_str());

                match (year, month, day, separator1, separator2) {
                    (Some(y), Some(mo), Some(d), Some(s1), Some(s2)) => format!(
                        "{}{}{}{}{}",
                        self.date.paint(y),
                        self.separator.paint(s1),
                        self.date.paint(mo),
                        self.separator.paint(s2),
                        self.date.paint(d)
                    ),
                    _ => input.to_string(),
                }
            })
            .to_string();

        self.regex_xx_xx_yyyy
            .replace_all(first_string.as_str(), |caps: &Captures<'_>| {
                let year = caps.name("year").map(|m| m.as_str());
                let month = caps.name("month").map(|m| m.as_str());
                let day = caps.name("day").map(|m| m.as_str());
                let separator1 = caps.name("separator1").map(|m| m.as_str());
                let separator2 = caps.name("separator2").map(|m| m.as_str());

                match (year, month, day, separator1, separator2) {
                    (Some(y), Some(mo), Some(d), Some(s1), Some(s2)) => format!(
                        "{}{}{}{}{}",
                        self.date.paint(y),
                        self.separator.paint(s1),
                        self.date.paint(mo),
                        self.separator.paint(s2),
                        self.date.paint(d)
                    ),
                    _ => input.to_string(),
                }
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
    fn test_date_dash_highlighter() {
        let config = DateTimeConfig::default();
        let highlighter = DateDashHighlighter::new(config).unwrap();

        let cases = vec![("2022-09-09", "2022-09-09"), ("No time here!", "No time here!")];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
