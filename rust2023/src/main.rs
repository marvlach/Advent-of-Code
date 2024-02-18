#![allow(dead_code)]

mod utils;
mod day16;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day16::part1(&input);
    day16::part2(&input);
}
