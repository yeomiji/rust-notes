// binary main functions cannot be tested
use adder; // needed since this file is in a separate crate

mod common;

// no need to add cfg(test) since the test directory is automatically excluded from
// compilation unless test is run
#[test]
fn it_adds_twoaaaaaaa() {
    common::setup();
    assert_eq!(4, adder::add_two(1));
}