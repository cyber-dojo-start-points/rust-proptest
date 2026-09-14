// multiplier is never read. Cargo.toml's "strict" feature denies warnings, so
// what would elsewhere be a note the learner could ignore stops the build.
pub fn answer() -> i32 {
    let multiplier = 6;
    6 * 7
}
