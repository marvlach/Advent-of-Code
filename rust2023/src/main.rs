#![allow(dead_code)]

mod utils;
mod day18;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day18::part1(&input);
    day18::part2(&input);
}
