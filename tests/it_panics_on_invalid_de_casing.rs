extern crate trybuild;

use trybuild::TestCases;

#[test]
fn it_panics_on_invalid_attr() {
    let t = TestCases::new();
    t.compile_fail("./tests/ui/*.rs");
}
