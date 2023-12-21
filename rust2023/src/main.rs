#![allow(dead_code)]

mod utils;
mod day11;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day11::part1(&input);
    day11::part2(&input);
}
