#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::{answer, digits_in_base};

#[test]
fn every_digit_of_the_answer_is_even() {
    for digit in digits_in_base(answer(), 10) {
        assert_eq!(0, digit % 2);
    }
}

#[test]
fn each_digit_of_the_answer_halves_the_one_before() {
    for pair in digits_in_base(answer(), 10).windows(2) {
        assert_eq!(pair[0], pair[1] * 2);
    }
}

// The one test of the three that does not hold. cargo stops after the target
// holding it, so the property in the other file never gets to run.
#[test]
fn the_answer_is_three_digits() {
    assert!(answer() > 99);
}
