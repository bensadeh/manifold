use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum HighlighterType {
    Foo,
    Bar,
}

#[derive(Debug)]
struct Highlighter {
    highlighter_type: HighlighterType,
    colors: Option<Vec<String>>,
}

// Define the Highlight trait
pub trait Highlight {
    fn apply(&self, input: String) -> String;
}

// Implement the Highlight trait for Highlighter
impl Highlight for Highlighter {
    fn apply(&self, input: String) -> String {
        // Example implementation of the apply method
        format!(
            "Applied {:?} highlighter with colors: {:?}",
            self.highlighter_type, self.colors
        )
    }
}

pub struct Manifold {
    highlighters: Vec<Arc<Box<dyn Highlight>>>,
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
            used_highlighters: HashSet::new(),
            error: None,
        }
    }

    // Method to set highlighters
    fn with_highlighters(mut self, highlighters: Vec<Arc<Box<dyn Highlight>>>) -> Self {
        self.highlighters = highlighters;
        self
    }

    pub fn apply(self, text: String) -> String {
        self.highlighters
            .into_iter()
            .fold(text, |acc, highlighter| highlighter.apply(acc))
    }
}

// Implement the Default trait for Manifold
impl Default for Manifold {
    fn default() -> Self {
        Manifold::new()
    }
}

struct ManifoldBuilder {
    highlighters: Vec<Arc<Box<dyn Highlight>>>,
    used_highlighters: HashSet<HighlighterType>,
    error: Option<String>,
}

impl ManifoldBuilder {
    fn with_new_foo_highlighter(mut self) -> Self {
        if self.used_highlighters.contains(&HighlighterType::Foo) {
            self.error = Some("Highlighter 'Foo' already added!".to_string());
        } else {
            self.highlighters.push(Arc::new(Box::new(Highlighter {
                highlighter_type: HighlighterType::Foo,
                colors: None,
            })));
            self.used_highlighters.insert(HighlighterType::Foo);
        }
        self
    }

    fn with_new_bar_highlighter_with_colors(mut self, colors: Vec<String>) -> Self {
        if self.used_highlighters.contains(&HighlighterType::Bar) {
            self.error = Some("Highlighter 'Bar' already added!".to_string());
        } else {
            self.highlighters.push(Arc::new(Box::new(Highlighter {
                highlighter_type: HighlighterType::Bar,
                colors: Some(colors),
            })));
            self.used_highlighters.insert(HighlighterType::Bar);
        }
        self
    }

    fn build(self) -> Result<Manifold, String> {
        if let Some(error) = self.error {
            Err(error)
        } else {
            Ok(Manifold::new().with_highlighters(self.highlighters))
        }
    }
}

fn main() {
    // Using the builder to create a Manifold
    let result = Manifold::builder()
        .with_new_foo_highlighter()
        .with_new_bar_highlighter_with_colors(vec!["red".to_string(), "blue".to_string()])
        .build();

    match result {
        Ok(manifold) => {
            let output = manifold.apply("Some text".to_string());
            println!("Output: {:?}", output);
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Using the default method to create a Manifold
    let default_manifold = Manifold::default();
}
