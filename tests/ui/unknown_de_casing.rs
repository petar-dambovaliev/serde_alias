extern crate serde_alias;

use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;

#[serde_alias(camelCase)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Foo {
    bar: String,
    bar1: String,
    bar2: String,
    bar3: String,
    bar4: String,
}

fn main() {}
