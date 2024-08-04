use std::sync::Arc;

use crate::error::Error;
use crate::highlighter::number::NumberHighlighter;
use crate::highlighter::quote::QuoteHighlighter;
use crate::highlighter::uuid::UuidHighlighter;
use crate::split_and_apply::apply_only_to_unhighlighted;
use crate::style::*;

pub trait Highlight: Sync + Send {
    fn apply(&self, input: &str) -> String;
}

pub struct Manifold {
    highlighters: Vec<Arc<dyn Highlight>>,
}

impl Manifold {
    const fn new() -> Self {
        Manifold {
            highlighters: Vec::new(),
        }
    }

    pub fn builder() -> ManifoldBuilder {
        ManifoldBuilder {
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

impl Default for Manifold {
    fn default() -> Self {
        Manifold::builder()
            .with_number_highlighter(None)
            .with_uuid_highlighter(None, None, None)
            .with_quote_highlighter(None, None)
            .build()
            .expect("Default Manifold construction should never fail.")
    }
}

pub struct ManifoldBuilder {
    highlighters: Vec<Arc<dyn Highlight>>,
    regex_errors: Vec<regex::Error>,
}

impl ManifoldBuilder {
    fn try_add_highlighter<T: Highlight + 'static>(mut self, highlighter: Result<T, regex::Error>) -> Self {
        match highlighter {
            Ok(h) => self.highlighters.push(Arc::new(h)),
            Err(e) => self.regex_errors.push(e),
        }

        self
    }

    pub fn with_number_highlighter(self, number: Option<Style>) -> Self {
        self.try_add_highlighter(NumberHighlighter::new(number.unwrap_or(cyan())))
    }

    pub fn with_uuid_highlighter(self, number: Option<Style>, letter: Option<Style>, dash: Option<Style>) -> Self {
        self.try_add_highlighter(UuidHighlighter::new(
            number.unwrap_or(blue_and_italic()),
            letter.unwrap_or(magenta_and_italic()),
            dash.unwrap_or(red()),
        ))
    }

    pub fn with_quote_highlighter(self, quotes_token: Option<char>, quote: Option<Style>) -> Self {
        self.try_add_highlighter(Ok(QuoteHighlighter::new(
            quotes_token.unwrap_or('"'),
            quote.unwrap_or(yellow()),
        )))
    }

    pub fn build(self) -> Result<Manifold, Error> {
        match self.regex_errors.is_empty() {
            true => Ok(Manifold::new().with_highlighters(self.highlighters)),
            false => Err(Error::RegexErrors(self.regex_errors)),
        }
    }
}
