#![cfg_attr(feature = "strict", deny(warnings))]

// A file in src/ is only compiled once a mod line names it, and these are
// those lines. Re-exporting keeps the names short for whoever uses them, so a
// test says hiker::answer rather than hiker::answer::answer.
mod answer;
mod digits;

pub use answer::answer;
pub use digits::digits_in_base;

// A #[cfg(test)] module inside src/lib.rs is the lib target's own test, so it
// runs in the first of the summary lines rather than in one of the ones the
// files in tests/ produce.
#[cfg(test)]
mod tests {
    #[test]
    fn the_answer_is_two_digits() {
        assert!(super::answer() > 9);
        assert!(super::answer() < 100);
    }
}
