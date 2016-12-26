
use std::io::BufRead;
use std::str::FromStr;

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
    let mut count = 0;
    let stdin = std::io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result.expect("expected line of data");
        let triangle = line.parse().expect("expected a triangle");
        if is_triangle(triangle) {
            count = count + 1
        }
    }
    println!("{}", count);
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
