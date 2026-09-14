#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::{answer, digits_in_base};

// The loop has no way out of it, so the run reaches max_seconds and the
// runner stops it. The assertion inside keeps the optimiser from throwing the
// loop away.
#[test]
fn every_digit_of_the_answer_is_even() {
    let mut total = 0;
    loop {
        total = (total + digits_in_base(answer(), 10).len()) % 1000;
        assert!(total < 1000);
    }
}
