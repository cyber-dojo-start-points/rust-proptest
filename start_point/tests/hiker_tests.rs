#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::answers;
use proptest::prelude::*;

proptest! {
    // count is not a value written here. proptest picks one from the range on
    // each of the many runs it gives the test, so what is written below has to
    // hold for every count in that range, not for one example of it.
    #[test]
    fn each_hiker_gets_one_answer(count in 0usize..20) {
        prop_assert_eq!(count, answers(count).len());
    }

    #[test]
    fn every_hiker_gets_42(count in 0usize..20) {
        for given in answers(count) {
            prop_assert_eq!(42, given);
        }
    }
}
