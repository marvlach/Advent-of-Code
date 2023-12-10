#![allow(dead_code)]

mod utils;
mod day09;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day09::part1(&input);
    day09::part2(&input);
}
