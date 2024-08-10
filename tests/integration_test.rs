use inlet_manifold::Highlighter;

#[test]
fn default_constructor_should_not_panic() {
    let result = std::panic::catch_unwind(Highlighter::default);

    assert!(result.is_ok(), "Default constructor should never fail");
}

#[test]
fn it_works() {
    let highlighter = Highlighter::default();

    let actual = highlighter.apply("Hello 123 world!".to_string());
    let expected = "Hello \u{1b}[36m123\u{1b}[0m world!".to_string();

    assert_eq!(actual, expected);
}
