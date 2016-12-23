
use std::cmp::{max, min};
use std::io::BufRead;

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
        state = match c {
            'U' => State(state.0, sub1(state.1)),
            'D' => State(state.0, add1(state.1)),
            'L' => State(sub1(state.0), state.1),
            'R' => State(add1(state.0), state.1),
            _ => panic!("Unknown character")
        };
    }
    (state, state.value())
}

fn decode_lines(input: &mut BufRead) -> Vec<u8> {
    let mut state = State(1, 1);
    let mut code = Vec::new();

    for line in input.lines() {
        let (s, v) = decode(state, &line.unwrap());
        state = s;
        code.push(v)
    }
    code
}

#[test]
fn aoc02_test1() {
    use std::io::Cursor;
    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    let mut input = Cursor::new(input);
    let code = decode_lines(&mut input);
    assert!(code == [1, 9, 8, 5]);
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let code = decode_lines(&mut input);

    println!("{:?}", code);
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
