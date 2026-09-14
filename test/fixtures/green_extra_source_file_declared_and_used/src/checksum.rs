/// The digits of n in base ten, added up.
pub fn checksum(n: i32) -> u32 {
    crate::digits_in_base(n, 10).into_iter().sum()
}
