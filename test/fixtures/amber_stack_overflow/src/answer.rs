// six() and seven() call each other, so answer() never returns. The recursion
// goes through two functions because a function that calls itself directly is
// caught by the unconditional_recursion lint, which --features strict turns
// into a build error.
fn six() -> i32 {
    seven() - 1
}

fn seven() -> i32 {
    six() + 1
}

pub fn answer() -> i32 {
    six() * 7
}
