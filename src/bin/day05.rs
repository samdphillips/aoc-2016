
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
        compute_password_try(&s)
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

fn compute_password_try(s: &str) -> String {
    format!("{:.7}", format!("{:x}", md5::compute(s)))
}

fn is_valid_try(s: &str) -> bool {
    s.starts_with("00000")
}

fn crack_password_pt1(s: &str) -> String {
    PasswordGenerator::new(s)
                      .filter(|p| is_valid_try(p))
                      .take(8)
                      .map(|s| s.chars().nth(5).unwrap())
                      .collect()
}

fn replace_null(vec: &Vec<u8>) -> Vec<u8> {
    let mut vec2 = Vec::new();

    for v in vec {
        let v = if *v == 0 { 32 } else { *v };
        vec2.push(v)
    }
    vec2
}

fn crack_password_pt2(s: &str) -> String {
    let lo = b'0';
    let hi = b'7';
    let mut passwd: [u8; 8] = [0; 8];
    let mut count = 0;
    let mut gen =
        PasswordGenerator::new(s)
                          .filter(|p| is_valid_try(p))
                          .map(|s| {
                              let b = s.as_bytes();
                              (b[5], b[6])
                          })
                          .filter(|&(i, _)| lo <= i && i <= hi)
                          .map(|(i, j)| (i - lo, j));

    while let Some((loc, val)) = gen.next() {
        let loc = loc as usize;
        if passwd[loc] == 0 {
            passwd[loc] = val;
            count += 1;
            println!("{}", String::from_utf8(replace_null(&passwd.to_vec())).unwrap());
        }

        if count == 8 {
            return String::from_utf8(passwd.to_vec()).expect("encoding weirdness happened")
        }
    }
    "".to_string()
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
    let flag = std::env::args().nth(1).unwrap();

    match flag.as_ref() {
        "-1" => println!("{}", crack_password_pt1("cxdnnyjw")),
        "-2" => println!("{}", crack_password_pt2("cxdnnyjw")),
        _ => println!("expected '-1' or '-2'")
    }

}
