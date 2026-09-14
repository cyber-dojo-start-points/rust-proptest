#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::digits_in_base;
use proptest::prelude::*;

proptest! {
    // prop_assume! throws away a value the property is not about. This one
    // throws away all but one value in a hundred thousand, so proptest gives
    // up before it ever gets a value to test. Nothing was proved, and nothing
    // failed either.
    #[test]
    fn the_digits_rebuild_the_number(n in 0i32..100_000, base in 2u32..=16) {
        prop_assume!(n % 100_000 == 99_999);
        let rebuilt = digits_in_base(n, base)
            .into_iter()
            .fold(0u32, |number, digit| number * base + digit);
        prop_assert_eq!(n, rebuilt as i32);
    }
}
