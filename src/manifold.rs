use std::collections::HashSet;

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

#[derive(Debug)]
pub struct Manifold {
    highlighters: Vec<Highlighter>,
}

impl Manifold {
    // Make the constructor private
    const fn new() -> Self {
        Manifold {
            highlighters: Vec::new(),
        }
    }

    // Public builder method to create a ManifoldBuilder
    fn builder() -> ManifoldBuilder {
        ManifoldBuilder {
            highlighters: Vec::new(),
            used_highlighters: HashSet::new(),
            error: None,
        }
    }

    // Method to set highlighters
    fn with_highlighters(mut self, highlighters: Vec<Highlighter>) -> Self {
        self.highlighters = highlighters;
        self
    }
}

// Implement the Default trait for Manifold
impl Default for Manifold {
    fn default() -> Self {
        Manifold::new()
    }
}

struct ManifoldBuilder {
    highlighters: Vec<Highlighter>,
    used_highlighters: HashSet<HighlighterType>,
    error: Option<String>,
}

impl ManifoldBuilder {
    fn with_new_foo_highlighter(mut self) -> Self {
        if self.used_highlighters.contains(&HighlighterType::Foo) {
            self.error = Some("Highlighter 'Foo' already added!".to_string());
        } else {
            self.highlighters.push(Highlighter {
                highlighter_type: HighlighterType::Foo,
                colors: None,
            });
            self.used_highlighters.insert(HighlighterType::Foo);
        }
        self
    }

    fn with_new_bar_highlighter_with_colors(mut self, colors: Vec<String>) -> Self {
        if self.used_highlighters.contains(&HighlighterType::Bar) {
            self.error = Some("Highlighter 'Bar' already added!".to_string());
        } else {
            self.highlighters.push(Highlighter {
                highlighter_type: HighlighterType::Bar,
                colors: Some(colors),
            });
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
        Ok(manifold) => println!("{:?}", manifold),
        Err(e) => println!("Error: {:?}", e),
    }

    // Using the default method to create a Manifold
    let default_manifold = Manifold::default();
    println!("{:?}", default_manifold);
}
