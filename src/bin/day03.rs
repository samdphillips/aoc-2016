
use std::io::BufRead;

fn is_triangle(nums: Vec<u32>) -> bool {
    2 * nums.iter().max().unwrap() < nums.iter().sum()
}

fn part_one() {
    let mut count = 0;
    let stdin = std::io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result.expect("expected line of data");
        let nums = line.split_whitespace()
                       .map(|s| s.parse().expect("expected integer string"))
                       .collect();
        if is_triangle(nums) {
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
