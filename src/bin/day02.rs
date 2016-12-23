
use std::cmp::{max, min};

fn sub1(x: u8) -> u8 {
    max(0, (x as i8) - 1) as u8
}

fn add1(x: u8) -> u8 {
    min(2, x + 1)
}

#[derive(Clone, Copy, Debug)]
struct State(u8, u8);

impl State {
    fn value(&self) -> u8 {
        self.0 + self.1 * 3 + 1
    }
}

fn decode(mut state: State, st: &str) -> (State, u8) {
    for c in st.chars() {
        print!("{:?}", state);
        state = match c {
            'U' => State(state.0, sub1(state.1)),
            'D' => State(state.0, add1(state.1)),
            'L' => State(sub1(state.0), state.1),
            'R' => State(add1(state.0), state.1),
            _ => panic!("Unknown character")
        };
        println!(" -[{}]-> {:?}", c, state);
    }

    println!("{:?} {}", state, state.value());
    (state, state.value())
}

fn part_one() {
    let state = State(1, 1);
    let (state, v) = decode(state, "ULL");
    let (state, v) = decode(state, "RRDDD");
    let (state, v) = decode(state, "LURDL");
    let (state, v) = decode(state, "UUUUD");
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
