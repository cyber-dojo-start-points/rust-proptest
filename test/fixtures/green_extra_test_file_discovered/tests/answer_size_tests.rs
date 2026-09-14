#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::answer;

// A test file the learner added. cargo makes a target out of every .rs file
// directly in tests/, so nothing has to be declared anywhere for this to run.
#[test]
fn the_answer_is_two_digits() {
    assert!(answer() > 9);
    assert!(answer() < 100);
}
