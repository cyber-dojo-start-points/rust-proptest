/// The digits of n written in the given base, most significant first.
///
/// digits_in_base(54, 13) is [4, 2], because fifty-four written in base
/// thirteen reads "42". That is Douglas Adams' joke: six times nine really
/// is forty-two, so long as nobody asks which base you were counting in.
pub fn digits_in_base(n: i32, base: u32) -> Vec<u32> {
    let mut left = n.unsigned_abs();
    if left == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while left > 0 {
        digits.push(left % base);
        left /= base;
    }
    digits.reverse();
    digits
}
