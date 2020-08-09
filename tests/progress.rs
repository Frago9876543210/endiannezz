#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failed/*.rs");
    t.pass("tests/success/*.rs");
}
