#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::answer;

// A second test file, which cargo makes a target of its own. It sorts before
// answer_tests, so it runs first and cargo stops after it, leaving the tests
// that would have passed unrun.
#[test]
fn the_answer_is_three_digits() {
    assert!(answer() > 99);
}
