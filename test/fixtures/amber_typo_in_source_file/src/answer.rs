// The function is called sevn and the call below says seven, which is close
// enough that rustc names the one the learner meant.
fn sevn() -> i32 {
    7
}

pub fn answer() -> i32 {
    6 * seven()
}
