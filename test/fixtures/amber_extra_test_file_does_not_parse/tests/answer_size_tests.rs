#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::answer;

// A second test file the learner started and left mid-sentence. Every .rs
// file directly in tests/ is a target of its own, so this one breaks the run
// even though the tests they finished are fine.
#[test]
fn the_answer_is_two_digits() {
    assert!(answer() > 9);
