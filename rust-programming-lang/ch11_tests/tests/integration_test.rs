use ch11_tests::add_two;
mod common;

// Each file in the tests directory is a separate crate Cargo will compile each of the files as an
// individual crate.), so we need to bring our library into each test crate’s scope. For that
// reason we add use adder; at the top of the code, which we didn’t need in the unit tests.
//
// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. Cargo treats
// the tests directory specially and compiles files in this directory only when we run cargo test.
#[test]
fn it_adds_two() {
    common::setupt();
    assert_eq!(4, add_two(2));
}
