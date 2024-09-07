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

    fn highlight_date(&self, caps: &Captures<'_>, input: &str) -> String {
        let year = caps.name("year").map(|m| m.as_str());
        let first = caps.name("first").map(|m| m.as_str());
        let second = caps.name("second").map(|m| m.as_str());
        let separator1 = caps.name("separator").map(|m| m.as_str());
        let separator2 = caps.name("separator2").map(|m| m.as_str());

        match (year, first, second, separator1, separator2) {
            (Some(y), Some(f), Some(s), Some(s1), Some(s2)) => format!(
                "{}{}{}{}{}",
                self.date.paint(y),
                self.separator.paint(s1),
                self.date.paint(f),
                self.separator.paint(s2),
                self.date.paint(s)
            ),
            _ => input.to_string(),
        }
    }
}

impl Highlight for DateDashHighlighter {
    fn apply(&self, input: &str) -> String {
        let first_run = self
            .regex_yyyy_xx_xx
            .replace_all(input, |caps: &Captures<'_>| self.highlight_date(caps, input))
            .to_string();

        self.regex_xx_xx_yyyy
            .replace_all(first_run.as_str(), |caps: &Captures<'_>| {
                self.highlight_date(caps, input)
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
