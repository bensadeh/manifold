pub use manifold::Manifold;

mod error;
mod highlighter;
pub mod manifold;
mod split_and_apply;
pub mod style;

pub const fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::tests::escape_code_converter::ConvertEscapeCodes;

    use super::*;

    pub(crate) mod escape_code_converter;

    #[test]
    fn it_works() {
        let manifold = Manifold::builder()
            .with_number_highlighter()
            .with_uuid_highlighter()
            .build()
            .unwrap();

        let actual = manifold.apply("Hello 123 world!".to_string());
        let expected = "Hello [cyan]123[reset] world!".to_string();
        assert_eq!(actual.convert_escape_codes(), expected);
    }
}
