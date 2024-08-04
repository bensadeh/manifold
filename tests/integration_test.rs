use inlet_manifold::manifold::Manifold;

#[test]
fn it_works() {
    let manifold = Manifold::default();

    let actual = manifold.apply("Hello 123 world!".to_string());
    let expected = "Hello \u{1b}[36m123\u{1b}[0m world!".to_string();

    assert_eq!(actual, expected);
}
