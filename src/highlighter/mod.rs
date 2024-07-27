pub mod number;
mod uuid;

pub trait Highlight {
    fn apply(&self, input: String) -> String;
}
