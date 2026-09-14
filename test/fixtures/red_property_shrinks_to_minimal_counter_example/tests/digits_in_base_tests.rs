#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::digits_in_base;
use proptest::prelude::*;

proptest! {
    // A property that is false, and false in a way with an edge to it: every
    // n below 100 has at most two digits and every n from 100 up has three.
    // proptest finds some failing n, then shrinks it down to that edge and
    // reports 100, which is the smallest n that breaks the property rather
    // than whichever one it happened to pick first.
    #[test]
    fn every_number_is_at_most_two_digits_long(n in 0i32..100_000) {
        prop_assert!(digits_in_base(n, 10).len() <= 2);
    }
}
