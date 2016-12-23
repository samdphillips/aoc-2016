
use std::cmp::{max, min};
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
struct Bounds(u8, u8);

trait Puzzle {
    fn bounds(&self, b: u8) -> Bounds;
}

fn sub1<P: Puzzle>(a: u8, b: u8, p: &P) -> u8 {
    let Bounds(low, _) = p.bounds(b);
    max(low as i8, (a as i8) - 1) as u8
}

fn add1<P: Puzzle>(a: u8, b: u8, p: &P) -> u8 {
    let Bounds(_, hi) = p.bounds(b);
    min(hi, a + 1)
}

#[derive(Clone, Copy, Debug)]
struct State(u8, u8);

static P1_KEYS: &'static str = "123456789";

impl State {
    fn value(&self) -> char {
        let i = self.0 + self.1 * 3;
        P1_KEYS.chars().nth(i as usize).unwrap()
    }
}

fn decode<P: Puzzle>(p: &P, mut state: State, st: &str) -> (State, char) {
    for c in st.chars() {
        state = match c {
            'U' => State(state.0, sub1(state.1, state.0, p)),
            'D' => State(state.0, add1(state.1, state.0, p)),
            'L' => State(sub1(state.0, state.1, p), state.1),
            'R' => State(add1(state.0, state.1, p), state.1),
            _ => panic!("Unknown character")
        };
    }
    (state, state.value())
}

fn decode_lines<P: Puzzle>(p: &P, input: &mut BufRead) -> String {
    let mut state = State(1, 1);
    let mut code = String::new();

    for line in input.lines() {
        let (s, v) = decode(p, state, &line.unwrap());
        state = s;
        code.push(v)
    }
    code
}


#[derive(Clone, Copy, Debug)]
struct Part1Puzzle { }

impl Puzzle for Part1Puzzle {
    #[allow(unused_variables)]
    fn bounds(&self, b: u8) -> Bounds {
        Bounds(0, 2)
    }
}

#[test]
fn aoc02_test1() {
    use std::io::Cursor;
    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    let mut input = Cursor::new(input);
    let p = Part1Puzzle { };
    let code = decode_lines(&p, &mut input);
    assert!(code == "1985");
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let p = Part1Puzzle { };
    let code = decode_lines(&p, &mut input);

    println!("{:?}", code);
}

#[derive(Clone, Copy, Debug)]
struct Part2Puzzle { }

impl Puzzle for Part2Puzzle {
    #[allow(unused_variables)]
    fn bounds(&self, b: u8) -> Bounds {
        Bounds(0, 2)
    }
}

#[test]
fn aoc02_test2() {
    use std::io::Cursor;
    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    let mut input = Cursor::new(input);
    let p = Part2Puzzle { };
    let code = decode_lines(&p, &mut input);
    assert!(code == "1985");
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
