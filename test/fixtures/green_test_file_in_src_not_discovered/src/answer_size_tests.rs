// A test file put in src/ rather than in tests/. No mod line names it, so
// cargo never compiles it and the test below never runs. cyber-dojo.sh
// compiles it on its own, with --test so the #[test] function is read, which
// is what stops a broken one from sitting here unnoticed.

use hiker::answer;

#[test]
fn the_answer_is_two_digits() {
    println!("checking the size of the answer");
    assert!(answer() > 9);
    assert!(answer() < 100);
}
