pub mod number;

pub trait Highlight {
    fn apply(&self, input: String) -> String;
}
