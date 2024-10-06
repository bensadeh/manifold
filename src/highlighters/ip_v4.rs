use crate::highlighter::Highlight;
use crate::IpV4Config;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

pub struct IpV4Highlighter {
    regex: Regex,
    number: NuStyle,
    separator: NuStyle,
}

impl IpV4Highlighter {
    pub fn new(config: IpV4Config) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)               # Enable verbose mode to allow comments and ignore whitespace
            (\b\d{1,3})          # Match 1 to 3 digits at a word boundary (start of the IP segment)
            (\.)                 # Match a literal dot (.)
            (\d{1,3})            # Match 1 to 3 digits (next IP segment)
            (\.)                 # Match a literal dot (.)
            (\d{1,3})            # Match 1 to 3 digits (next IP segment)
            (\.)                 # Match a literal dot (.)
            (\d{1,3}\b)          # Match 1 to 3 digits at a word boundary (end of the IP segment)
    ",
        )?;

        Ok(Self {
            regex,
            number: config.number.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for IpV4Highlighter {
    fn apply(&self, input: &str) -> String {
        let segment = &self.number;
        let separator = &self.separator;
        let highlight_groups = [
            (segment, 1),
            (separator, 2),
            (segment, 3),
            (separator, 4),
            (segment, 5),
            (separator, 6),
            (segment, 7),
        ];

        self.regex
            .replace_all(input, |caps: &Captures<'_>| {
                let mut output = String::new();
                for &(color, group) in &highlight_groups {
                    output.push_str(&format!("{}", color.paint(&caps[group])));
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
    use crate::IpV4Config;

    use super::*;

    #[test]
    fn test_ip_v4_highlighter() {
        let highlighter = IpV4Highlighter::new(IpV4Config {
            number: Style::new().fg(Color::Blue),
            separator: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            (
                "10.0.0.123",
                "[blue]10[reset][red].[reset][blue]0[reset][red].[reset][blue]0[reset][red].[reset][blue]123[reset]",
            ),
            (
                "192.168.0.1",
                "[blue]192[reset][red].[reset][blue]168[reset][red].[reset][blue]0[reset][red].[reset][blue]1[reset]",
            ),
            ("Invalid regex: 192.168.0", "Invalid regex: 192.168.0"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
