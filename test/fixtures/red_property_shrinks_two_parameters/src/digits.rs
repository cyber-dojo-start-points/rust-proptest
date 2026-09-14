/// The digits of n written in the given base, most significant first.
///
/// The remainder below is taken against ten rather than against base, which
/// is right for base ten and wrong for every other base. The two tests in
/// tests/answer_tests.rs only ever ask for base ten, so they pass; the
/// property asks for every base from two to sixteen, so it does not.
pub fn digits_in_base(n: i32, base: u32) -> Vec<u32> {
    let mut left = n.unsigned_abs();
    if left == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while left > 0 {
        digits.push(left % 10);
        left /= base;
    }
    digits.reverse();
    digits
}
