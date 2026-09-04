#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::digits_in_base;
use proptest::prelude::*;

proptest! {
    // A property, about every value rather than one. n and base are ranges
    // rather than values: proptest chooses a pair from them on each of the
    // many runs it gives the test, so what is written below has to hold for
    // all of them. When it does not, proptest shrinks the pair to the
    // smallest that still fails and reports that one.
    #[test]
    fn the_digits_rebuild_the_number(n in 0i32..100_000, base in 2u32..=16) {
        let rebuilt = digits_in_base(n, base)
            .into_iter()
            .fold(0u32, |number, digit| number * base + digit);
        prop_assert_eq!(n, rebuilt as i32);
    }
}
