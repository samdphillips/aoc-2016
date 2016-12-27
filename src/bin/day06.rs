
use std::collections::HashMap;
use std::io::BufRead;

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

    fn top(&self) -> char {
        let mut entries: Vec<(char, u32)> = self.counts.iter().map(|(ch, c)| (*ch, *c)).collect();
        entries.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        entries[0].0
    }

    fn bottom(&self) -> char {
        let mut entries: Vec<(char, u32)> = self.counts.iter().map(|(ch, c)| (*ch, *c)).collect();
        entries.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
        entries[0].0
    }
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut cs: Vec<Counter> = (0..8).map(|_| Counter::new()).collect();

    for line in stdin.lock().lines() {
        for (i, ch) in line.unwrap().chars().enumerate() {
            cs[i].add_char(ch)
        }
        println!("{}", cs.iter().map(|c| c.top()).collect::<String>());
    }
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut cs: Vec<Counter> = (0..8).map(|_| Counter::new()).collect();

    for line in stdin.lock().lines() {
        for (i, ch) in line.unwrap().chars().enumerate() {
            cs[i].add_char(ch)
        }
        println!("{}", cs.iter().map(|c| c.bottom()).collect::<String>());
    }
}

fn main () {
    let flag = std::env::args().nth(1).unwrap();

    match flag.as_ref() {
        "-1" => part_one(),
        "-2" => part_two(),
        _ => println!("expected '-1' or '-2'")
    }
}
