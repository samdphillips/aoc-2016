
use std::cmp::{max, min};

fn sub1(x: u8) -> u8 {
    max(0, (x as i8) - 1) as u8
}

fn add1(x: u8) -> u8 {
    min(2, x + 1)
}


fn decode(s: &str) {
    let mut state : (u8, u8) = (1, 1);

    for c in "ULL".chars() {
        state = match c {
            'U' => (state.0, sub1(state.1)),
            'D' => (state.0, add1(state.1)),
            'L' => (sub1(state.0), state.1),
            'R' => (add1(state.0), state.1),
            _ => panic!("Unknown character")
        }
    }

    println!("{:?} {}", state, state.0 + state.1 * 3 + 1);
}

fn part_one() {
    decode("ULL")
}

fn part_two() { }

fn main() {
    let flag = std::env::args().nth(1).unwrap();

    match flag.as_ref() {
        "-1" => part_one(),
        "-2" => part_two(),
        _ => println!("expected '-1' or '-2'")

    }
}
