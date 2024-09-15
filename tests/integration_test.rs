use inlet_manifold::{Color, Highlighter, NumberConfig, QuoteConfig, Style, UuidConfig};

#[test]
fn default_constructor_should_not_panic() {
    let result = std::panic::catch_unwind(Highlighter::default);

    assert!(result.is_ok(), "Default constructor should never fail");
}

#[test]
fn it_works() {
    let mut builder = Highlighter::builder();

    builder
        .with_number_highlighter(NumberConfig {
            number: Style {
                fg: Some(Color::Cyan),
                ..Style::default()
            },
        })
        .with_quote_highlighter(QuoteConfig {
            quotes_token: '"',
            color: Style {
                fg: Some(Color::Yellow),
                ..Style::default()
            },
        })
        .with_uuid_highlighter(UuidConfig::default());

    let highlighter = match builder.build() {
        Ok(h) => h,
        Err(_) => panic!("Failed to build highlighter"),
    };

    let actual = highlighter.apply("Hello 123 world! ".to_string());
    let expected = "Hello \u{1b}[36m123\u{1b}[0m world! ".to_string();

    assert_eq!(actual, expected);
}
