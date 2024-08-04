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
            errors: Vec::new(),
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
            .with_number_highlighter()
            .with_uuid_highlighter()
            .build()
            .expect("Default Manifold construction should never fail.")
    }
}

pub struct ManifoldBuilder {
    highlighters: Vec<Arc<dyn Highlight>>,
    errors: Vec<regex::Error>,
}

impl ManifoldBuilder {
    fn try_add_highlighter<T: Highlight + 'static>(mut self, highlighter: Result<T, regex::Error>) -> Self {
        match highlighter {
            Ok(h) => self.highlighters.push(Arc::new(h)),
            Err(e) => self.errors.push(e),
        }

        self
    }

    pub fn with_number_highlighter(self) -> Self {
        self.try_add_highlighter(NumberHighlighter::new(cyan()))
    }

    pub fn with_number_highlighter_from_style(self, style: Style) -> Self {
        self.try_add_highlighter(NumberHighlighter::new(style))
    }

    pub fn with_uuid_highlighter(self) -> Self {
        self.try_add_highlighter(UuidHighlighter::new(blue_and_italic(), magenta_and_italic(), red()))
    }

    pub fn with_uuid_highlighter_from_style(self, number: Style, letter: Style, dash: Style) -> Self {
        self.try_add_highlighter(UuidHighlighter::new(number, letter, dash))
    }

    pub fn with_quote_highlighter(mut self) -> Self {
        let highlighter = QuoteHighlighter::new('"', yellow());
        self.highlighters.push(Arc::new(highlighter));

        self
    }

    pub fn with_quote_highlighter_from_style(mut self, quotes_token: char, style: Style) -> Self {
        let highlighter = QuoteHighlighter::new(quotes_token, style);
        self.highlighters.push(Arc::new(highlighter));

        self
    }

    pub fn build(self) -> Result<Manifold, Error> {
        match self.errors.is_empty() {
            true => Ok(Manifold::new().with_highlighters(self.highlighters)),
            false => Err(Error::RegexErrors(self.errors)),
        }
    }
}
