use tests; // each file in the tests directory is a separate crate, 
           // so we need to bring our library into each test crate’s scope

           // tests is found inte Cargo.toml, name of the project

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests::add_two(2));
}