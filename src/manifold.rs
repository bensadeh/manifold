use std::sync::Arc;

use crate::highlighter::number::NumberHighlighter;
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
    }
}

pub struct ManifoldBuilder {
    highlighters: Vec<Arc<dyn Highlight>>,
}

impl ManifoldBuilder {
    pub fn with_number_highlighter(mut self) -> Self {
        let number_highlighter = NumberHighlighter::new(cyan());

        self.highlighters.push(Arc::new(number_highlighter));

        self
    }

    pub fn with_number_highlighter_from_style(mut self, style: Style) -> Self {
        let number_highlighter = NumberHighlighter::new(style);

        self.highlighters.push(Arc::new(number_highlighter));

        self
    }

    pub fn with_uuid_highlighter(mut self) -> Self {
        let number_highlighter = UuidHighlighter::new(blue_and_italic(), magenta_and_italic(), red());

        self.highlighters.push(Arc::new(number_highlighter));

        self
    }

    pub fn with_uuid_highlighter_from_style(mut self, number: Style, letter: Style, dash: Style) -> Self {
        let number_highlighter = UuidHighlighter::new(number, letter, dash);

        self.highlighters.push(Arc::new(number_highlighter));

        self
    }

    pub fn build(self) -> Manifold {
        Manifold::new().with_highlighters(self.highlighters)
    }
}
