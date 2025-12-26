mod common;

use writing_automated_test::*;
use common::setup;
#[test]
fn it_workX() {
    setup();
    let result = add(2, 2);
    assert_eq!(result, 4);
}