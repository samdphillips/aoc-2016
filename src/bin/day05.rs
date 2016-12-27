
extern crate md5;

struct PasswordGenerator {
    seed: String,
    counter: u32,
}

impl PasswordGenerator {
    fn new(seed: &str) -> PasswordGenerator {
        PasswordGenerator { seed: seed.to_string(), counter: 0 }
    }

    fn compute_try(&self) -> String {
        let mut s = self.seed.clone();
        s.push_str(&format!("{}", self.counter));
        compute_password_try_pt1(&s)
    }

    fn next_try(&mut self) {
        self.counter += 1
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let v = self.compute_try();
        self.next_try();
        Some(v)
    }
}

fn compute_password_try_pt1(s: &str) -> String {
    format!("{:.6}", format!("{:x}", md5::compute(s)))
}

fn is_valid_try(s: &str) -> bool {
    s.starts_with("00000")
}

fn crack_password(s: &str) -> String {
    PasswordGenerator::new(s)
                      .filter(|p| is_valid_try(p))
                      .take(8)
                      .map(|s| s.chars().last().unwrap())
                      .collect()
}

#[test]
fn aoc05_test_compute_password_try() {
    assert!(compute_password_try("abc3231929").starts_with("00000"));
    assert!(!compute_password_try("abc3231928").starts_with("00000"));
}

#[test]
fn aoc05_test_compute_try() {
    let x = PasswordGenerator::new("abc");
    assert!(x.compute_try() == "577571")
}

#[test]
fn aoc05_test_password_generator_iterator() {
    let mut x = PasswordGenerator { seed: "abc".to_string(), counter: 3231928 }
                    .filter(|p| is_valid_try(p));
    assert_eq!(x.next(), Some("000001".to_string()))
}

#[test]
fn aoc05_test_password_generator_pt1() {
    assert_eq!(crack_password("abc"), "18f47a30");
}

fn main() {
    println!("{}", crack_password("cxdnnyjw"))
}
