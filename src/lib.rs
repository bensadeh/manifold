pub use crate::config::*;
pub use crate::error::Error;
pub use crate::highlighter::Highlighter;
pub use crate::style::Color;
pub use crate::style::Style;

pub mod config;
mod error;
pub mod highlighter;
mod highlighters;
mod normalizer;
mod split_and_apply;
pub mod style;

#[cfg(test)]
mod tests {
    pub(crate) mod escape_code_converter;
}
