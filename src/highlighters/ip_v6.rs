use crate::highlighter::Highlight;
use crate::IpV6Config;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

pub struct IpV6Highlighter {
    regex: Regex,
    number: NuStyle,
    letter: NuStyle,
    separator: NuStyle,
}

impl IpV6Highlighter {
    pub fn new(config: IpV6Config) -> Result<Self, Error> {
        let regex = Regex::new(r#"(?x) (?:[0-9a-fA-F]{1,4}:{1,2}) {3,}[0-9a-fA-F]{1,4}"#)?;

        Ok(Self {
            regex,
            number: config.number.into(),
            letter: config.letter.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for IpV6Highlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| {
                let text = &caps[0];
                if !text.chars().any(|c| matches!(c, 'a'..='f' | 'A'..='F')) {
                    // If no hexadecimal letters are found, return the original text unmodified
                    text.to_string()
                } else {
                    // Apply highlighting as before
                    text.chars()
                        .map(|c| match c {
                            '0'..='9' => self.number.paint(c.to_string()).to_string(),
                            'a'..='f' | 'A'..='F' => self.letter.paint(c.to_string()).to_string(),
                            ':' | '.' => self.separator.paint(c.to_string()).to_string(),
                            _ => c.to_string(),
                        })
                        .collect::<String>()
                }
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
    fn test_ip_v4_highlighter() {
        let highlighter = IpV6Highlighter::new(IpV6Config {
            number: Style::new().fg(Color::Blue),
            letter: Style::new().fg(Color::Yellow),
            separator: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            (
                "2001:db8:0:0:0:ff00:42:8329",
                "[blue]2[reset][blue]0[reset][blue]0[reset][blue]1[reset][red]:[reset][yellow]d[reset][yellow]b[reset][blue]8[reset][red]:[reset][blue]0[reset][red]:[reset][blue]0[reset][red]:[reset][blue]0[reset][red]:[reset][yellow]f[reset][yellow]f[reset][blue]0[reset][blue]0[reset][red]:[reset][blue]4[reset][blue]2[reset][red]:[reset][blue]8[reset][blue]3[reset][blue]2[reset][blue]9[reset]"
            ),
            (
                "2001:db8::ff00:42:8329",
                "[blue]2[reset][blue]0[reset][blue]0[reset][blue]1[reset][red]:[reset][yellow]d[reset][yellow]b[reset][blue]8[reset][red]:[reset][red]:[reset][yellow]f[reset][yellow]f[reset][blue]0[reset][blue]0[reset][red]:[reset][blue]4[reset][blue]2[reset][red]:[reset][blue]8[reset][blue]3[reset][blue]2[reset][blue]9[reset]"
            ),
            ("Not ipv4: 192.168.0.1", "Not ipv4: 192.168.0.1"),
            ("11:47:39:850", "11:47:39:850"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
