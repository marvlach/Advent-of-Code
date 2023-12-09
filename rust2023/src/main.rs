#![allow(dead_code)]

mod utils;
mod day08;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day08::part1(&input);
    day08::part2(&input);
}
