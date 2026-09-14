// No `mod checksum;` line names this file, so cargo never compiles it as part
// of the crate. cyber-dojo.sh compiles it on its own instead, which is what
// stops the missing brace below from sitting here unnoticed while the tests
// go green.

pub fn checksum(n: i32) -> i32 {
    n % 10
