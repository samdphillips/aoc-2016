
use std::str::{Chars, FromStr};

#[derive(Debug, PartialEq)]
enum Dir { N, S, E, W }

impl Dir {
    fn turn(&self, turn: Turn) -> Dir {
        let d = match self { &Dir::N => 0, &Dir::E => 1, &Dir::S => 2, &Dir::W => 3 };
        let t = match turn { Turn::Left => -1, Turn::Right => 1 };

        match ((d + t % 4) + 4) % 4 {
            0 => Dir::N,
            1 => Dir::E,
            2 => Dir::S,
            3 => Dir::W,
            x => panic!("got direction: {}", x)
        }
    }
}

#[derive(Debug, PartialEq)]
enum Turn { Left, Right }

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Turn, String> {
        match s {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err("Unknown turn".to_string())
        }
    }
}

#[derive(Debug, PartialEq)]
struct Command {
    turn: Turn,
    steps: u32
}

#[derive(Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    facing: Dir
}

impl Position {
    fn update(&self, cmd: Command) -> Position {
        let dir = self.facing.turn(cmd.turn);
        let (dx, dy) =
            match dir {
                Dir::N => (0, 1),
                Dir::S => (0, -1),
                Dir::E => (1, 0),
                Dir::W => (-1, 0)
            };

        let dx = dx * cmd.steps as i32;
        let dy = dy * cmd.steps as i32;

        Position { x: self.x + dx, y: self.y + dy, facing: dir }
    }

    fn distance_to_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

struct Tokenize<'a> {
    inner: Chars<'a>
}

impl<'a> Tokenize<'a> {
    fn from_str(s: &'a str) -> Tokenize<'a> {
        Tokenize { inner: s.chars() }
    }
}

impl<'a> Iterator for Tokenize<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            let mut opt_ch = self.inner.next();
            if opt_ch.is_none() { return None }

            let mut ch = opt_ch.unwrap();
            if ch == ',' || ch == ' ' {
                continue
            }

            if ch == 'L' || ch == 'R' {
                return Some(ch.to_string())
            }

            if ch.is_digit(10) {
                let mut s = String::new();
                loop {
                    s.push(ch);
                    opt_ch = self.inner.next();
                    if opt_ch.is_none() || !opt_ch.unwrap().is_digit(10) {
                        return Some(s)
                    }
                    ch = opt_ch.unwrap();
                }
            }
        }
    }
}

fn read_command(tok: &mut Tokenize) -> Option<Command> {
    let turn = tok.next();
    if turn.is_none() { return None }

    let steps = tok.next();
    if steps.is_none() { return None }

    let turn : Turn = turn.unwrap().parse().expect("expected a turn");
    let steps : u32 = steps.unwrap().parse().expect("expected a u32");
    Some(Command { turn: turn, steps: steps })
}

#[test]
fn aoc01_test_turns() {
    assert!(Dir::N.turn(Turn::Left) == Dir::W);
    assert!(Dir::N.turn(Turn::Right) == Dir::E);

    assert!(Dir::E.turn(Turn::Left) == Dir::N);
    assert!(Dir::E.turn(Turn::Right) == Dir::S);

    assert!(Dir::S.turn(Turn::Left) == Dir::E);
    assert!(Dir::S.turn(Turn::Right) == Dir::W);

    assert!(Dir::W.turn(Turn::Left) == Dir::S);
    assert!(Dir::W.turn(Turn::Right) == Dir::N);
}

#[test]
fn aoc01_test_update_position() {
    let cmd = Command { turn: Turn::Left, steps: 5 };
    let posn = Position { x: 0, y: 0, facing: Dir::N };
    assert!(posn.update(cmd) == Position { x: -5, y: 0, facing: Dir::W });
}

#[test]
fn aoc01_test_tokenize() {
    let mut tok = Tokenize::from_str(",  L   ,   32");
    assert!(tok.next() == Some("L".to_string()));
    assert!(tok.next() == Some("32".to_string()));
    assert!(tok.next() == None);
}

#[test]
fn aoc01_test_read_command() {
    let mut tok = Tokenize::from_str("L32, R2, L5");
    assert!(read_command(&mut tok) == Some(Command { turn: Turn::Left, steps: 32 }));
    assert!(read_command(&mut tok) == Some(Command { turn: Turn::Right, steps: 2 }));
    assert!(read_command(&mut tok) == Some(Command { turn: Turn::Left, steps: 5 }));
    assert!(read_command(&mut tok) == None)
}

#[test]
fn aoc01_test_distance() {
    let posn = Position { x: 2, y: -2, facing: Dir::N };
    assert!(posn.distance_to_origin() == 4);
}

fn main () {
}
