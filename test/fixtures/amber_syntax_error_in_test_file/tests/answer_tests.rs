#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::{answer, digits_in_base};

// The closing brace of the test is missing. cargo makes a target out of this
// file, so the whole run stops here and the property in the other file never
// gets to run either.
#[test]
fn every_digit_of_the_answer_is_even() {
    for digit in digits_in_base(answer(), 10) {
        assert_eq!(0, digit % 2);
    }
