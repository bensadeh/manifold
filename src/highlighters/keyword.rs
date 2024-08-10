use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

use crate::highlighter::Highlight;
use crate::KeywordConfig;

pub struct KeywordHighlighter {
    regex: Regex,
    style: NuStyle,
}

impl KeywordHighlighter {
    pub fn new(keyword_config: KeywordConfig) -> Result<Self, Error> {
        let keyword_pattern = keyword_config
            .words
            .iter()
            .map(|word| regex::escape(word))
            .collect::<Vec<_>>()
            .join("|");

        let regex = Regex::new(&format!(r"\b({})\b", keyword_pattern))?;

        Ok(Self {
            regex,
            style: keyword_config.style.into(),
        })
    }
}

impl Highlight for KeywordHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| match self.style.background {
                None => {
                    format!("{}", self.style.paint(&caps[0]))
                }
                Some(_) => {
                    let capture_with_extra_padding = format!(" {} ", &caps[0]);
                    format!("{}", self.style.paint(capture_with_extra_padding))
                }
            })
            .to_string()
    }
}
