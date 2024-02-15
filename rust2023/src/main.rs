#![allow(dead_code)]

mod utils;
mod day13;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day13::part1(&input);
    day13::part2(&input);
}
