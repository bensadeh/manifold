use crate::config::*;
use crate::error::Error;
use crate::highlighters::number::NumberHighlighter;
use crate::highlighters::quote::QuoteHighlighter;
use crate::highlighters::uuid::UuidHighlighter;
use crate::split_and_apply::apply_only_to_unhighlighted;
use std::sync::Arc;

pub trait Highlight: Sync + Send {
    fn apply(&self, input: &str) -> String;
}

pub struct Highlighter {
    highlighters: Vec<Arc<dyn Highlight>>,
}

impl Highlighter {
    const fn new() -> Self {
        Highlighter {
            highlighters: Vec::new(),
        }
    }

    pub fn builder() -> HighlightBuilder {
        HighlightBuilder {
            highlighters: Vec::new(),
            regex_errors: Vec::new(),
        }
    }

    fn with_highlighters(mut self, highlighters: Vec<Arc<dyn Highlight>>) -> Self {
        self.highlighters = highlighters;

        self
    }

    pub fn apply(self, text: String) -> String {
        self.highlighters
            .into_iter()
            .fold(text, |acc, highlighter| apply_only_to_unhighlighted(&acc, highlighter))
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Highlighter::builder()
            .with_number_highlighter(NumberConfig::default())
            .with_uuid_highlighter(UuidConfig::default())
            .with_quote_highlighter(QuoteConfig::default())
            .build()
            .expect("Default Manifold construction should never fail.")
    }
}

pub struct HighlightBuilder {
    highlighters: Vec<Arc<dyn Highlight>>,
    regex_errors: Vec<regex::Error>,
}

impl HighlightBuilder {
    fn try_add_highlighter<T: Highlight + 'static>(mut self, highlighter: Result<T, regex::Error>) -> Self {
        match highlighter {
            Ok(h) => self.highlighters.push(Arc::new(h)),
            Err(e) => self.regex_errors.push(e),
        }

        self
    }

    pub fn with_number_highlighter(self, config: NumberConfig) -> Self {
        self.try_add_highlighter(NumberHighlighter::new(config))
    }

    pub fn with_uuid_highlighter(self, config: UuidConfig) -> Self {
        self.try_add_highlighter(UuidHighlighter::new(config))
    }

    pub fn with_quote_highlighter(self, config: QuoteConfig) -> Self {
        self.try_add_highlighter(Ok(QuoteHighlighter::new(config)))
    }

    pub fn build(self) -> Result<Highlighter, Error> {
        match self.regex_errors.is_empty() {
            true => Ok(Highlighter::new().with_highlighters(self.highlighters)),
            false => Err(Error::RegexErrors(self.regex_errors)),
        }
    }
}
