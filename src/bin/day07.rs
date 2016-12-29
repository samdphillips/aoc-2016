
use std::io::BufRead;

fn is_tls_address(b: &[u8]) -> bool {
    let mut inside = false;
    let mut result = false;

    for i in 3..b.len() {
        let w = b[i-3];
        let x = b[i-2];
        let y = b[i-1];
        let z = b[i];
        if z == w && z != y && y == x {
            if inside {
                return false
            }
            result = true;
        }

        match z {
            b'[' => inside = true,
            b']' => inside = false,
            _ => ()
        }
    }
    result
}

struct Match (u8, u8, bool);

impl Match {
    fn is_inverse(&self, other: &Match) -> bool {
        self.0 == other.1 && self.1 == other.0 && self.2 != other.2
    }
}

struct SslMatches (Vec<Match>);

impl SslMatches {
    fn new() -> SslMatches {
        SslMatches(Vec::new())
    }

    fn contains_inverse(&self, m: &Match) -> bool {
        self.0.iter().any(|n| m.is_inverse(n))
    }

    fn add_match(&mut self, m: Match) {
        self.0.push(m)
    }
}

fn is_ssl_address(b: &[u8]) -> bool {
    let mut inside = false;
    let mut matches = SslMatches::new();

    for i in 2..b.len() {
        match b[i] {
            b'[' => inside = true,
            b']' => inside = false,
            z => {
                let x = b[i-2];
                let y = b[i-1];

                if z == x {
                    let m = Match(x, y, inside);
                    if matches.contains_inverse(&m) {
                        return true
                    }
                    matches.add_match(m)
                }
            }
        }
    }

    false
}

#[test]
fn aoc07_test_ssl_matchers() {
    let mut matches = SslMatches::new();
    let a = Match(0, 1, false);
    assert!(!matches.contains_inverse(&a));
    matches.add_match(a);
    let b = Match(1, 0, true);
    assert!(matches.contains_inverse(&b));
}

#[test]
fn aoc07_test_ssl_address_matches() {
    // aba[bab]xyz supports SSL
    // (aba outside square brackets with corresponding bab within square brackets).
    assert!(is_ssl_address(b"aba[bab]xyz"));

    // xyx[xyx]xyx does not support SSL
    // (xyx, but no corresponding yxy).
    assert!(!is_ssl_address(b"xyx[xyx]xyx"));

    // aaa[kek]eke supports SSL
    // (eke in supernet with corresponding kek in hypernet; the aaa sequence is not related,
    // because the interior character must be different).
    assert!(is_ssl_address(b"aaa[kek]eke"));

    // zazbz[bzb]cdb supports SSL
    // (zaz has no corresponding aza, but zbz has a corresponding bzb, even though zaz
    // and zbz overlap).
    assert!(is_ssl_address(b"zazbz[bzb]cdb"));
}

#[test]
fn aoc07_test_tls_address_matches() {
    assert!(is_tls_address(b"abba[mnop]qrst"));
    assert!(!is_tls_address(b"abcd[bddb]xyyx"));
    assert!(!is_tls_address(b"aaaa[qwer]tyui"));
    assert!(is_tls_address(b"ioxxoj[asdfgh]zxcvbn"));
}

fn part_one() {
    let stdin = std::io::stdin();
    let n =
        stdin.lock()
             .lines()
             .map(|s| s.unwrap())
             .filter(|s| {
                 is_tls_address(s.as_bytes())
             })
             .count();

    println!("{}", n)
}

fn part_two() {
    let stdin = std::io::stdin();
    let n =
        stdin.lock()
             .lines()
             .map(|s| s.unwrap())
             .filter(|s| {
                 is_ssl_address(s.as_bytes())
             })
             .count();

    println!("{}", n)
}

fn main () {
    let flag = std::env::args().nth(1).unwrap();

    match flag.as_ref() {
        "-1" => part_one(),
        "-2" => part_two(),
        _ => println!("expected '-1' or '-2'")
    }
}
