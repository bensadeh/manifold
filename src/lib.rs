pub use errors::ManifoldError;
pub use manifold::Manifold;

pub mod errors;
mod highlighter;
pub mod manifold;

pub const fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
