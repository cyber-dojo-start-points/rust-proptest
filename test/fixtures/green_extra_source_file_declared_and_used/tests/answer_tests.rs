#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::{answer, checksum, digits_in_base};

// Ordinary tests, about one value rather than about every value.
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

// checksum() lives in src/checksum.rs, which src/lib.rs declares with a mod
// line, so cargo compiles it and this test can reach it.
#[test]
fn the_digits_of_the_answer_add_up_to_six() {
    assert_eq!(6, checksum(answer()));
}
