#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::answer;

// cargo makes a test target out of each tests/*.rs, so a file one level down
// is never built by cargo and this test never runs. cyber-dojo.sh compiles it
// on its own so that it cannot sit here broken unnoticed, and that check
// builds it without running it.
#[test]
fn the_answer_is_two_digits() {
    println!("checking the size of the answer");
    assert!(answer() > 9);
    assert!(answer() < 100);
}
