extern crate serde_alias;

use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;

#[serde_alias(
    CamelCase,
    PascalCase,
    LowerCase,
    UpperCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase
)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Foo {
    bar: String,
    bar1: String,
    bar2: String,
    bar3: String,
    bar4: String,
}

#[test]
fn de_camel_pascal() {
    let got: Foo = serde_json::from_str(
        r#"{"Bar": "Bar", "bar1": "bar1", "Bar2": "Bar2", "bar_3": "bar_3", "bar-4": "bar-4"}"#,
    )
    .unwrap();

    let expected = Foo {
        bar: "Bar".to_string(),
        bar1: "bar1".to_string(),
        bar2: "Bar2".to_string(),
        bar3: "bar_3".to_string(),
        bar4: "bar-4".to_string(),
    };

    assert_eq!(got, expected);
}
