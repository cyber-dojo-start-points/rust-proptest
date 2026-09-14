#![cfg_attr(feature = "strict", deny(warnings))]

// A file in src/ is only compiled once a mod line names it, and these are
// those lines. Re-exporting keeps the names short for whoever uses them, so a
// test says hiker::answer rather than hiker::answer::answer.
mod answer;
mod digits;

pub use answer::answer;
pub use digits::digits_in_base;
