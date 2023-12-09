use crate::utils::lcm;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let mut lines = input.lines().filter(|l| !l.is_empty());
    let instructions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Direction::from_char(c))
        .collect();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let mut split = line.split("=");
        let from = split.next().unwrap().trim();
        let tos = split
            .next()
            .unwrap()
            .trim()
            .chars()
            .filter(|x| x.is_ascii_alphanumeric())
            .collect::<String>();
        let to = tos.split_at(3);
        nodes.insert(from.to_owned(), (to.0.to_owned(), to.1.to_owned()));
    }
    (instructions, nodes)
}

pub fn part1(input: &str) {
    // 19637
    let (instructions, nodes) = parse_input(input);
    let mut current = &String::from("AAA");
    let mut steps: u64 = 0;
    for d in instructions.iter().cycle() {
        if current == "ZZZ" {
            break;
        }
        steps += 1;
        current = match d {
            Direction::L => &nodes.get(current).unwrap().0,
            Direction::R => &nodes.get(current).unwrap().1,
        };
    }
    println!("{steps}");
}

pub fn part2(input: &str) {
    // 8811050362409
    let (instructions, nodes) = parse_input(input);
    let start_nodes = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<&String>>();
    let mut periods: Vec<u64> = Vec::new();
    for node in start_nodes {
        let mut running = node;
        let mut h0: HashMap<(u64, &String), u64> = HashMap::new();
        let mut steps: u64 = 0;
        for (i, d) in instructions.iter().enumerate().cycle() {
            if h0.contains_key(&(i as u64, running)) {
                break;
            }
            if running.ends_with("Z") {
                h0.insert((i as u64, running), steps);
            }
            steps += 1;
            running = match d {
                Direction::L => &nodes.get(running).unwrap().0,
                Direction::R => &nodes.get(running).unwrap().1,
            };
        }
        if h0.len() > 1 {
            panic!("It's only one anyway:)");
        }
        periods.push(*h0.values().next().unwrap());
    }
    let ans = periods
        .iter()
        .fold(periods[0], |prev, curr| lcm(prev, *curr));
    println!("{:?}", ans);
}
