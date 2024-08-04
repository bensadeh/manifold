pub use crate::manifold::Highlighter;

mod error;
mod highlighter;
pub mod manifold;
mod split_and_apply;
pub mod style;

#[cfg(test)]
mod tests {
    pub(crate) mod escape_code_converter;
}
