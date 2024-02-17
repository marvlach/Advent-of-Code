#![allow(dead_code)]

mod utils;
mod day15;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day15::part1(&input);
    day15::part2(&input);
}
