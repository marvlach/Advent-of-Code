#![allow(dead_code)]

mod utils;
mod day10;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day10::part1(&input);
    day10::part2(&input);
}
