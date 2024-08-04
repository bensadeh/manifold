use std::sync::Arc;

use crate::highlighter::number::NumberHighlighter;
use crate::highlighter::uuid::UuidHighlighter;
use crate::style::{Color, Style};

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

    fn builder() -> ManifoldBuilder {
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
            .fold(text, |acc, highlighter| highlighter.apply(acc.as_str()))
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

struct ManifoldBuilder {
    highlighters: Vec<Arc<dyn Highlight>>,
}

impl ManifoldBuilder {
    fn with_number_highlighter(mut self) -> Self {
        let style = Style {
            fg: Some(Color::Cyan),
            ..Style::default()
        };

        let number_highlighter = NumberHighlighter::new(style);

        self.highlighters.push(Arc::new(number_highlighter));

        self
    }

    fn with_number_highlighter_from_style(mut self, style: Style) -> Self {
        let number_highlighter = NumberHighlighter::new(style);

        self.highlighters.push(Arc::new(number_highlighter));

        self
    }

    fn with_uuid_highlighter(mut self) -> Self {
        let number = Style {
            fg: Some(Color::Blue),
            italic: true,
            ..Style::default()
        };
        let letter = Style {
            fg: Some(Color::Magenta),
            italic: true,
            ..Style::default()
        };
        let dash = Style {
            fg: Some(Color::Red),
            ..Style::default()
        };

        let number_highlighter = UuidHighlighter::new(number, letter, dash);

        self.highlighters.push(Arc::new(number_highlighter));

        self
    }

    fn with_uuid_highlighter_from_style(mut self, number: Style, letter: Style, dash: Style) -> Self {
        let number_highlighter = UuidHighlighter::new(number, letter, dash);

        self.highlighters.push(Arc::new(number_highlighter));

        self
    }

    fn build(self) -> Manifold {
        Manifold::new().with_highlighters(self.highlighters)
    }
}

fn main() {
    // Using the builder to create a Manifold
    let manifold = Manifold::builder()
        .with_number_highlighter()
        .with_uuid_highlighter()
        .build();

    let converted = manifold.apply("text".to_string());

    println!("{}", converted);
}
