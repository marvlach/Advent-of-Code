#![allow(dead_code)]

mod day01;
mod day02;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    //day01::part1(&input);
    //day01::part2(&input);
    day02::part1(&input);
    day02::part2(&input);
}
