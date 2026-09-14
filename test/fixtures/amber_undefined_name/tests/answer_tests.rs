#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::digits_in_base;

// The crate exports answer(), and this asks for a name it does not have.
#[test]
fn every_digit_of_the_answer_is_even() {
    for digit in digits_in_base(hiker::answer_to_everything(), 10) {
        assert_eq!(0, digit % 2);
    }
}
