
use std::cmp::{max, min};
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
struct Bounds(u8, u8);

trait Puzzle {
    fn init_state(&self) -> State;
    fn bounds(&self, b: u8) -> Bounds;
    fn value(&self, s: &State) -> char;
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
    (state, p.value(&state))
}

fn decode_lines<P: Puzzle>(p: &P, input: &mut BufRead) -> String {
    let mut state = p.init_state();
    let mut code = String::new();

    for line in input.lines() {
        let (s, v) = decode(p, state, &line.unwrap());
        state = s;
        println!("{:?} {}", state, v);
        code.push(v)
    }
    code
}


#[derive(Clone, Copy, Debug)]
struct Part1Puzzle { }

static P1_KEYS: &'static str = "123456789";

impl Puzzle for Part1Puzzle {
    fn init_state(&self) -> State {
        State(1, 1)
    }

    #[allow(unused_variables)]
    fn bounds(&self, b: u8) -> Bounds {
        Bounds(0, 2)
    }

    fn value(&self, s: &State) -> char {
        let i = s.0 + s.1 * 3;
        P1_KEYS.chars().nth(i as usize).unwrap()
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

#[derive(Clone, Copy, Debug)]
struct Part2Puzzle { }

static P2_KEYS: &'static str = "XX1XXX234X56789XABCXXXDXX";

impl Puzzle for Part2Puzzle {
    fn init_state(&self) -> State {
        State(0, 2)
    }

    #[allow(unused_variables)]
    fn bounds(&self, b: u8) -> Bounds {
        let b = (b as i8 - 2).abs();
        match b {
            0 => Bounds(0, 4),
            1 => Bounds(1, 3),
            2 => Bounds(2, 2),
            _ => panic!("shouldn't happen {}", b)
        }
    }

    fn value(&self, s: &State) -> char {
        let i = s.0 + s.1 * 5;
        P2_KEYS.chars().nth(i as usize).unwrap()
    }
}

#[test]
fn aoc02_test2() {
    use std::io::Cursor;
    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    let mut input = Cursor::new(input);
    let p = Part2Puzzle { };
    let code = decode_lines(&p, &mut input);
    assert!(code == "5DB3");
}

fn solve<P: Puzzle>(p: &P) {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let code = decode_lines(p, &mut input);
    println!("{}", code);
}

fn main() {
    let flag = std::env::args().nth(1).unwrap();

    match flag.as_ref() {
        "-1" => solve(&Part1Puzzle { }),
        "-2" => solve(&Part2Puzzle { }),
        _ => println!("expected '-1' or '-2'")
    }
}
