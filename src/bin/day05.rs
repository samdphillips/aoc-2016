
extern crate md5;

fn compute_password_try(s: &str) -> String {
    format!("{:.6}", format!("{:x}", md5::compute(s)))
}

#[test]
fn aoc05_test_compute_password_try() {
    assert!(compute_password_try("abc3231929").starts_with("00000"));
    assert!(!compute_password_try("abc3231928").starts_with("00000"));
}

fn main() {
}
