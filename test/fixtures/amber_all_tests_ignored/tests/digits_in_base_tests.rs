#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::digits_in_base;
use proptest::prelude::*;

proptest! {
    // The property is parked too. proptest writes each of its tests as an
    // ordinary #[test] function, so #[ignore] parks one the same way it parks
    // any other.
    #[test]
    #[ignore]
    fn the_digits_rebuild_the_number(n in 0i32..100_000, base in 2u32..=16) {
        let rebuilt = digits_in_base(n, base)
            .into_iter()
            .fold(0u32, |number, digit| number * base + digit);
        prop_assert_eq!(n, rebuilt as i32);
    }
}
