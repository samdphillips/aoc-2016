
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fmt;
use std::io::BufRead;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Command {
    Rect(usize, usize),
    RotateR { row: usize, amt: usize },
    RotateC { col: usize, amt: usize }
}
impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Command, String> {
        lazy_static! {
            static ref RECT_PAT: Regex = Regex::new(r"^rect\s+(?P<w>\d+)x(?P<h>\d+)$").unwrap();
            static ref ROTATE_PAT: Regex = Regex::new(r"^rotate\s+(?P<axis>row|column)\s+(x|y)=(?P<index>\d+)\s+by\s+(?P<amount>\d+)$").unwrap();
        }

        let cap = RECT_PAT.captures(s);
        if cap.is_some() {
            let cap = cap.unwrap();
            let w = (&cap["w"]).parse().unwrap();
            let h = (&cap["h"]).parse().unwrap();
            Ok(Command::Rect(w, h))
        } else {
            match ROTATE_PAT.captures(s) {
                Some(cap) => {
                    let axis = &cap["axis"];
                    let index = (&cap["index"]).parse().unwrap();
                    let amount = (&cap["amount"]).parse().unwrap();
                    match axis {
                        "row" => Ok(Command::RotateR { row: index, amt: amount }),
                        "column" => Ok(Command::RotateC { col: index, amt: amount }),
                        _ => Err("invalid parse".to_string())
                    }
                }
                None => Err("invalid parse".to_string())
            }
        }
    }
}

struct Display(usize, usize, Vec<bool>);

impl Display {
    fn new(w: usize, h: usize) -> Display {
        let mut cells = Vec::with_capacity(w * h);
        for _ in 0..(w * h) {
            cells.push(false)
        }
        Display(w, h, cells)
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        for j in 0..self.1 {
            for i in 0..self.0 {
                self[(i, j)] = false
            }
        }
    }

    fn column(&self, col: usize) -> Vec<bool> {
        let mut v = Vec::with_capacity(self.1);
        for i in 0..self.1 {
            v.push(self[(col, i)])
        }
        v
    }

    fn row(&self, row: usize) -> Vec<bool> {
        let mut v = Vec::with_capacity(self.0);
        for i in 0..self.0 {
            v.push(self[(i, row)])
        }
        v
    }

    fn count_lit(&self) -> usize {
        self.2.iter().filter(|&&v| v).count()
    }

    fn exec(&mut self, cmd: Command) {
        match cmd {
            Command::Rect(w, h) => self.rect(w, h),
            Command::RotateC { col, amt } => self.rotate_col(col, amt),
            Command::RotateR { row, amt } => self.rotate_row(row, amt),
        }
    }

    fn rect(&mut self, w: usize, h: usize) {
        for i in 0..w {
            for j in 0..h {
                self[(i, j)] = true
            }
        }
    }

    fn rotate_col(&mut self, col: usize, amt: usize) {
        let orig_col = self.column(col);
        for j1 in 0..self.1 {
            let j0 = ((j1 as i8 - amt as i8) + self.1 as i8) as usize % self.1;
            self[(col, j1)] = orig_col[j0]
        }
    }

    fn rotate_row(&mut self, row: usize, amt: usize) {
        let orig_row = self.row(row);
        for j1 in 0..self.0 {
            let j0 = ((j1 as i8 - amt as i8) + self.0 as i8) as usize % self.0;
            self[(j1, row)] = orig_row[j0]
        }
    }

    fn index_of(&self, x: usize, y: usize) -> usize {
        self.0 * y + x
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for j in 0..self.1 {
            for i in 0..self.0 {
                let s = if self[(i,j)] { "#" } else { "." };
                try!(f.write_str(s));
            }
            try!(f.write_str("\n"));
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Display {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &bool {
        &self.2[self.index_of(x, y)]
    }
}

impl IndexMut<(usize, usize)> for Display {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut bool {
        let i = self.index_of(x, y);
        &mut self.2[i]
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut d = Display::new(50, 6);
    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let cmd: Command = line.parse().unwrap();
        d.exec(cmd);
        println!("---\n{:?}", d);
    }
    println!("{}", d.count_lit());
}

#[test]
fn aoc08_test_parse_line() {
    assert_eq!("rect 3x2".parse(), Ok(Command::Rect(3, 2)));
    assert_eq!("rotate column x=1 by 1".parse(), Ok(Command::RotateC { col: 1, amt: 1 }));
    assert_eq!("rotate row y=0 by 4".parse(), Ok(Command::RotateR { row: 0, amt: 4 }));
    assert!("foobarbaz".parse::<Command>().is_err());
}

#[test]
fn aoc08_test_display_commands() {
    let mut d = Display::new(7, 3);
    d[(0,0)] = true;
    assert_eq!(format!("{:?}", d), "#......\n.......\n.......\n");
    d.clear();

    d.rect(3, 2);
    assert_eq!(format!("{:?}", d), "###....\n###....\n.......\n");

    d.rotate_col(1, 1);
    assert_eq!(format!("{:?}", d), "#.#....\n###....\n.#.....\n");

    d.rotate_row(0, 4);
    assert_eq!(format!("{:?}", d), "....#.#\n###....\n.#.....\n");

    d.rotate_col(1, 1);
    assert_eq!(format!("{:?}", d), ".#..#.#\n#.#....\n.#.....\n");

    assert_eq!(d.count_lit(), 6);
}
