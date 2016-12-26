
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
struct Counter {
    counts: HashMap<char, u32>
}

impl Counter {
    fn new() -> Counter {
        Counter { counts: HashMap::new() }
    }

    fn add_char(&mut self, ch: char) {
        let c = self.counts.entry(ch).or_insert(0);
        *c += 1;
    }

    fn key(&self) -> String {
        let mut entries: Vec<(char, u32)> = self.counts.iter().map(|(ch, c)| (*ch, *c)).collect();
        // hack because ordering_chaining not in stable
        entries.sort_by(|&(cha, a), &(chb, b)| {
                            let ord = b.cmp(&a);
                            if ord == Ordering::Equal {
                                cha.cmp(&chb)
                            } else {
                                ord
                            }
                        });
        entries.iter().take(5).map(|&(ch, _)| ch).collect()
    }
}

#[derive(Debug)]
struct RoomCode {
    name: Vec<String>,
    sector: u32,
    key: String
}

impl RoomCode {
    fn compute_key(&self) -> String {
        let mut counter = Counter::new();
        for c in self.name.iter().flat_map(|s| s.chars()) {
            counter.add_char(c)
        }
        counter.key()
    }

    fn valid_code(&self) -> bool {
        self.compute_key() == self.key
    }
}

impl FromStr for RoomCode {
    type Err = String;

    #[allow(unused_variables)]
    fn from_str(s: &str) -> Result<RoomCode, String> {
        let mut segments: Vec<String> = s.split('-').map(|s| s.to_string()).collect();
        let sector_key = try!(segments.pop().ok_or("empty string"));
        let j = try!(sector_key.find('[').ok_or("missing '['"));
        let k = try!(sector_key.find(']').ok_or("missing ']'"));
        let sector: u32 = try!(sector_key[0..j].parse().map_err(|e| "not an integer sector code"));
        let key = sector_key[j+1..k].to_string();

        Ok(RoomCode { name: segments, sector: sector, key: key })
    }
}

#[test]
fn aoc04_test_counter_add_char() {
    let mut c = Counter::new();
    let ach = 'a';
    let bch = 'b';
    c.add_char(ach);
    assert!(c.counts[&ach] == 1);
    c.add_char(ach);
    assert!(c.counts[&ach] == 2);
    c.add_char(bch);
    assert!(c.counts[&bch] == 1);
}

#[test]
fn aoc04_test_counter_key() {
    let mut c = Counter::new();
    let ach = 'a';
    let bch = 'b';
    let cch = 'c';
    c.add_char(cch);
    c.add_char(ach);
    c.add_char(ach);
    c.add_char(bch);
    c.add_char(bch);
    c.add_char(bch);
    c.add_char(cch);
    c.add_char(bch);
    println!("{}", c.key());
    assert!(c.key() == "bac");
}

#[test]
fn aoc04_test_keyname() {
    let mut c = Counter::new();
    for ch in "notarealroom".chars() {
        c.add_char(ch)
    }
    println!("{}", c.key());
    assert!(c.key() == "oarel");
}

#[test]
fn aoc04_test_parse_room_code() {
    let s = "aaaaa-bbb-z-y-x-123[abxyz]";
    let room_code: RoomCode = s.parse().expect("room code");
    println!("{:?}", room_code);
    assert!(room_code.key == "abxyz");
    assert!(room_code.sector == 123);
    assert!(room_code.name.len() == 5);
}

#[test]
fn aoc04_test_room_code_key() {
    let n = vec!("aaaaa".to_string(),
                 "bbb".to_string(),
                 "z".to_string(),
                 "y".to_string(),
                 "x".to_string());
    let r = RoomCode { name: n, sector: 123, key: "abxyz".to_string() };
    assert!(r.key == r.compute_key());
}

#[test]
fn aoc04_test_room_code_validity() {
    assert!("aaaaa-bbb-z-y-x-123[abxyz]".parse::<RoomCode>().unwrap().valid_code());
    assert!("a-b-c-d-e-f-g-h-987[abcde]".parse::<RoomCode>().unwrap().valid_code());
    assert!("not-a-real-room-404[oarel]".parse::<RoomCode>().unwrap().valid_code());
    assert!(!"totally-real-room-200[decoy]".parse::<RoomCode>().unwrap().valid_code());
}

fn main () {
    let stdin = std::io::stdin();
    let x : u32 =
        stdin.lock()
             .lines()
             .map(|s| s.unwrap().parse::<RoomCode>().unwrap())
             .filter(|rc| rc.valid_code())
             .map(|rc| rc.sector)
             .sum();

     println!("{:?}", x);
}
