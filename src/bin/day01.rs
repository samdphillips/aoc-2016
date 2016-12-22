
#![allow(unused_must_use)]

use std::collections::HashMap;
use std::io::Read;
use std::str::{Chars, FromStr};

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Command {
    CmdTurn(Turn),
    CmdStep
}

#[derive(Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    facing: Dir
}

impl Position {
    fn origin() -> Position {
        Position { x: 0, y: 0, facing: Dir::N }
    }

    fn update(&self, cmd: Command) -> Position {
        match cmd {
            Command::CmdTurn(turn) => self.update_turn(turn),
            Command::CmdStep => self.update_step()
        }
    }

    fn update_turn(&self, turn: Turn) -> Position {
        Position { x: self.x, y: self.y, facing: self.facing.turn(turn) }
    }

    fn update_step(&self) -> Position {
        let (dx, dy) =
            match self.facing {
                Dir::N => (0, 1),
                Dir::S => (0, -1),
                Dir::E => (1, 0),
                Dir::W => (-1, 0)
            };

        Position { x: self.x + dx, y: self.y + dy, facing: self.facing }
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

struct Parse<'a> {
    tokenize: Tokenize<'a>,
    steps: u32,
}


impl<'a> Parse<'a> {
    fn next_tokens(&mut self) -> Option<Command> {
        let turn = self.tokenize.next();
        if turn.is_none() { return None }

        let steps = self.tokenize.next();
        if steps.is_none() { return None }

        let turn : Turn = turn.unwrap().parse().expect("expected a turn");
        let steps : u32 = steps.unwrap().parse().expect("expected a u32");
        self.steps = steps;
        Some(Command::CmdTurn(turn))
    }
}

impl<'a> Iterator for Parse<'a> {
    type Item = Command;

    fn next(&mut self) -> Option<Command> {
        if self.steps == 0 {
            self.next_tokens()
        } else {
            self.steps = self.steps - 1;
            Some(Command::CmdStep)
        }
    }
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
    let cmd = Command::CmdTurn(Turn::Left);
    let posn = Position { x: 0, y: 0, facing: Dir::N };
    let mut posn = posn.update(cmd);
    for _ in 0..5 {
        posn = posn.update(Command::CmdStep)
    }
    assert!(posn == Position { x: -5, y: 0, facing: Dir::W });
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
    let tok = Tokenize::from_str("L2, R1, L1");
    let mut parse = Parse { tokenize: tok, steps: 0 };
    assert!(parse.next() == Some(Command::CmdTurn(Turn::Left)));
    assert!(parse.next() == Some(Command::CmdStep));
    assert!(parse.next() == Some(Command::CmdStep));
    assert!(parse.next() == Some(Command::CmdTurn(Turn::Right)));
    assert!(parse.next() == Some(Command::CmdStep));
    assert!(parse.next() == Some(Command::CmdTurn(Turn::Left)));
    assert!(parse.next() == Some(Command::CmdStep));
    assert!(parse.next() == None)
}

#[test]
fn aoc01_test_distance() {
    let posn = Position { x: 2, y: -2, facing: Dir::N };
    assert!(posn.distance_to_origin() == 4);
}

fn part_one () {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer);
    let tok = Tokenize::from_str(&buffer);
    let parse = Parse { tokenize: tok, steps: 0 };
    let posn = parse.fold(Position::origin(), |posn, cmd| posn.update(cmd));

    println!("{:?} {}", posn, posn.distance_to_origin());
}

fn part_two() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer);
    let tok = Tokenize::from_str(&buffer);
    let parse = Parse { tokenize: tok, steps: 0 };
    let mut table = HashMap::new();
    let mut posn = Position::origin();

    table.insert((0,0), 1);
    for cmd in parse {
        posn = posn.update(cmd);

        match cmd {
            Command::CmdStep => {
                let p = (posn.x, posn.y);
                let c = table.get(&p).unwrap_or(&0) + 1;
                if c == 2 {
                    println!("solution: {:?} {}", posn, posn.distance_to_origin());
                    break
                }
                table.insert(p, c);
            },
            _ => ()
        }
    }
}

fn main() {
    let flag = std::env::args().nth(1).unwrap();

    match flag.as_ref() {
        "-1" => part_one(),
        "-2" => part_two(),
        _ => println!("expected '-1' or '-2'")

    }
}
