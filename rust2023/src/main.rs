#![allow(dead_code)]

mod utils;
mod day12;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day12::part1(&input);
    day12::part2(&input);
}
