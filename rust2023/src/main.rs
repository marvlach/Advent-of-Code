#![allow(dead_code)]

mod utils;
mod day17;
fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    day17::part1(&input);
    day17::part2(&input);
}
