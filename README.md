# serde_alias

An attribute macro to add deserializing serde casing aliases for all fields in a struct

Simply list which aliase casing types should be applied

```rust
extern crate serde_alias;

use serde::{Deserialize};
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
#[derive(Deserialize)]
struct Foo {
    bar: String,
}
```
