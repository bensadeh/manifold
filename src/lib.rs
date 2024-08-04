pub use manifold::Manifold;

mod highlighter;
pub mod manifold;
mod split_and_apply;
pub mod style;

pub const fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) mod escape_code_converter;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
