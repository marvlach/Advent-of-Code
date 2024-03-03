#![allow(dead_code)]

mod utils;
mod day19;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day19::part1(&input);
    day19::part2(&input);
}
