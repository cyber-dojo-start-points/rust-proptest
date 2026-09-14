#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::digits_in_base;
use proptest::prelude::*;

proptest! {
    // prop_assume! throws away a value the property is not about, and
    // proptest fetches another one in its place. This one throws away about
    // half of them, which leaves plenty, so the property still gets its full
    // count of passing runs.
    #[test]
    fn an_even_number_ends_in_an_even_digit(n in 0i32..100_000) {
        prop_assume!(n % 2 == 0);
        let digits = digits_in_base(n, 10);
        prop_assert_eq!(0, digits[digits.len() - 1] % 2);
    }
}
