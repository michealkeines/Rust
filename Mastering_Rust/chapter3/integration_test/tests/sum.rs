use integration_test::sum;

mod common;
/*
when we write integration tests, we use the crate that's being tested, like any other 
user of the library would use it
*/

use common::{setup, teardown};

#[test]
fn sum_test() {
    assert_eq!(sum(6,8), 14);
}

#[test]
fn test_with_fixture() {
    setup();
    assert_eq!(sum(7, 14), 21);
    teardown();
}