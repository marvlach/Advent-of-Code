use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64,
}

fn parse_line_part_1(line: &str) -> Vec<String> {
    line.split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}

fn parse_line_part_2(line: &str) -> Vec<String> {
    vec![line
        .split(":")
        .last()
        .unwrap()
        .trim()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()]
}

fn parse_input(input: &str, line_parser: fn(&str) -> Vec<String>) -> Vec<Race> {
    let mut lines = input.lines();
    let mut races: Vec<Race> = Vec::new();

    let times = line_parser(lines.next().unwrap());
    let dists = line_parser(lines.next().unwrap());
    for (t, d) in zip(times, dists) {
        races.push(Race {
            time: t.parse().unwrap(),
            dist: d.parse().unwrap(),
        })
    }
    races
}

fn run(races: Vec<Race>) -> usize {
    races
        .iter()
        .map(|race| {
            (1..race.time)
                .map(|time_hold_button| time_hold_button * (race.time - time_hold_button))
                .filter(|x| x > &race.dist)
                .count()
        })
        .product()
}

pub fn part1(input: &str) {
    let races: Vec<Race> = parse_input(input, parse_line_part_1);
    let ans = run(races);
    println!("{ans}");
}

pub fn part2(input: &str) {
    let races = parse_input(input, parse_line_part_2);
    let ans = run(races);
    println!("{:?}", ans);
}
