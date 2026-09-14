// A panic of the learner's own, marking a place they have not filled in yet.
// cargo reports it as FAILED exactly as it reports a failing assertion, and
// the panic message is the only thing telling the two apart.
pub fn answer() -> i32 {
    panic!("the computer has not finished thinking yet");
}
