#![allow(dead_code)]

mod utils;
mod day14;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day14::part1(&input);
    day14::part2(&input);
}
