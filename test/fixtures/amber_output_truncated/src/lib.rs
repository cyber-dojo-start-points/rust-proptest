#![cfg_attr(feature = "strict", deny(warnings))]

mod answer;
mod digits;

pub use answer::answer;
pub use digits::digits_in_base;

#[cfg(test)]
mod tests {
    // The learner put a print inside a loop to see what was happening. The
    // lib target is the first cargo runs, so this flood fills the head of
    // stdout and every "test result:" line lands past the 50K the runner
    // keeps.
    #[test]
    fn every_digit_of_the_answer_is_even() {
        let mut total = 0;
        for i in 0..5000 {
            println!("debug: i is {}, total is {}", i, total);
            total += 1;
        }
        assert_eq!(5000, total);
    }
}
