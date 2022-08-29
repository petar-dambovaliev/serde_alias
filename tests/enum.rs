use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Foo<V> {
    var1: V,
    var2: V,
    var3: V,
}

macro_rules! assert_valid_json_casing {
    ($json:expr, $case:ident) => {
        #[serde_alias($case)]
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        enum FooVar {
            Bar,
            Baz,
            FooBar,
        }

        let actual: Foo<FooVar> = serde_json::from_str($json).expect("valid JSON");

        let expected = Foo {
            var1: FooVar::Bar,
            var2: FooVar::Baz,
            var3: FooVar::FooBar,
        };

        assert_eq!(actual, expected);
    };
}

#[test]
fn camel() {
    let json = r#"{"var1": "bar", "var2": "baz", "var3": "fooBar"}"#;
    assert_valid_json_casing!(json, CamelCase);
}

#[test]
fn pascal() {
    let json = r#"{"var1": "Bar", "var2": "Baz", "var3": "FooBar"}"#;
    assert_valid_json_casing!(json, PascalCase);
}

#[test]
fn lower() {
    let json = r#"{"var1": "bar", "var2": "baz", "var3": "foo bar"}"#;
    assert_valid_json_casing!(json, LowerCase);
}

#[test]
fn upper() {
    let json = r#"{"var1": "BAR", "var2": "BAZ", "var3": "FOO BAR"}"#;
    assert_valid_json_casing!(json, UpperCase);
}

#[test]
fn snake() {
    let json = r#"{"var1": "bar", "var2": "baz", "var3": "foo_bar"}"#;
    assert_valid_json_casing!(json, SnakeCase);
}

#[test]
fn screaming_snake() {
    let json = r#"{"var1": "BAR", "var2": "BAZ", "var3": "FOO_BAR"}"#;
    assert_valid_json_casing!(json, ScreamingSnakeCase);
}

#[test]
fn kebab() {
    let json = r#"{"var1": "bar", "var2": "baz", "var3": "foo-bar"}"#;
    assert_valid_json_casing!(json, KebabCase);
}

#[test]
fn screaming_kebab() {
    let json = r#"{"var1": "BAR", "var2": "BAZ", "var3": "FOO-BAR"}"#;
    assert_valid_json_casing!(json, ScreamingKebabCase);
}
