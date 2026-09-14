pub fn answer() -> i32 {
    // A print the learner added to watch the code run. eprintln! goes to
    // stderr, which is a different stream from the one the test summaries go
    // to, and the rag-lambda reads both.
    eprintln!("answer was called");
    6 * 7
}
