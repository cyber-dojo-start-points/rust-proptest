#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer() -> i32 {
    6 * 9
}

pub fn answers(count: usize) -> Vec<i32> {
    vec![answer(); count]
}
