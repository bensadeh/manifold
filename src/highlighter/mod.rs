pub mod number;
mod uuid;

pub trait Highlight: Sync + Send {
    fn apply(&self, input: String) -> String;
}
