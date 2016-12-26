
extern crate itertools;

use std::io::BufRead;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Triangle(u32, u32, u32);

impl Triangle {
    fn iter(&self) -> std::vec::IntoIter<u32> {
        vec!(self.0, self.1, self.2).into_iter()
    }
}

impl FromStr for Triangle {
    type Err = String;

    fn from_str(s: &str) -> Result<Triangle, Self::Err> {
        let (a, b, c) = try!(parse_line(s));
        Ok(Triangle(a, b, c))
    }
}

fn parse_line(s: &str) -> Result<(u32, u32, u32), String> {
    let nums_str: Vec<&str> = s.split_whitespace().collect();

    if nums_str.len() == 3 {
        let nums: Vec<u32> = nums_str.iter()
                                       .map(|s| s.parse().expect("expected integer string"))
                                       .collect();
        Ok((nums[0], nums[1], nums[2]))
    } else {
        Err("Expected 3 space separated values".to_string())
    }
}

fn is_triangle(t: Triangle) -> bool {
    2 * t.iter().max().unwrap() < t.iter().sum()
}

fn part_one() {
    let stdin = std::io::stdin();
    let count = stdin.lock()
                     .lines()
                     .map(|r| r.expect("expected line of data"))
                     .map(|l| parse_line(&l).expect("expected three integers"))
                     .map(|(a, b, c)| Triangle(a, b, c))
                     .filter(|t| is_triangle(*t))
                     .count();
    println!("{}", count);
}

fn part_two() {
    let stdin = std::io::stdin();
    let count = stdin.lock()
                     .lines()
                     .map(|r| r.expect("expected line of data"))
                     .map(|l| parse_line(&l).expect("expected three integers"))
                     .tuples()
                     .flat_map(|((a0, b0, c0),
                                 (a1, b1, c1),
                                 (a2, b2, c2))|
                               vec!(Triangle(a0, a1, a2),
                                    Triangle(b0, b1, b2),
                                    Triangle(c0, c1, c2)).into_iter())
                     .filter(|t| is_triangle(*t))
                     .count();
    println!("{}", count);
}

fn main() {
    let flag = std::env::args().nth(1).unwrap();

    match flag.as_ref() {
        "-1" => part_one(),
        "-2" => part_two(),
        _ => println!("expected '-1' or '-2'")
    }
}
