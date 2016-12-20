
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
}

fn read_turn(input: Chars) -> Option<Turn> {
    let mut input = input.skip_while(|c| *c == ' ' || *c == ',');

    match input.next() {
        Some('L') => Some(Turn::Left),
        Some('R') => Some(Turn::Right),
        _ => None
    }
}

fn read_steps(input: Chars) -> Option<u32> {
    let s : String = input.take_while(|c| c.is_numeric()).collect();
    u32::from_str(&s).ok()
}

#[test]
fn test_turns() {
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
fn test_update_position() {
    let cmd = Command { turn: Turn::Left, steps: 5 };
    let posn = Position { x: 0, y: 0, facing: Dir::N };
    assert!(posn.update(cmd) == Position { x: -5, y: 0, facing: Dir::W });
}

#[test]
fn test_read_turn() {
    assert!(read_turn("L".chars()) == Some(Turn::Left));
    assert!(read_turn("R".chars()) == Some(Turn::Right));
    assert!(read_turn("   R".chars()) == Some(Turn::Right));
    assert!(read_turn(",  R".chars()) == Some(Turn::Right));
    assert!(read_turn("".chars()) == None);
}

#[test]
fn test_read_steps() {
    assert!(read_steps("1234".chars()) == Some(1234));
    assert!(read_steps("1234, L1233".chars()) == Some(1234));
}

fn main () {
}
