use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Clone, Copy, Debug)]
enum PipeType {
    NS,
    WE,
    NE,
    NW,
    SE,
    SW,
}

impl PipeType {
    fn from_char(c: &char) -> PipeType {
        match c {
            '|' => PipeType::NS,
            '-' => PipeType::WE,
            'L' => PipeType::NE,
            'J' => PipeType::NW,
            '7' => PipeType::SW,
            'F' => PipeType::SE,
            _ => panic!("Invalid {c}"),
        }
    }

    fn can_go(&self, pos: (usize, usize)) -> [(usize, usize); 2] {
        match &self {
            PipeType::NS => [(pos.0 + 1, pos.1), (pos.0 - 1, pos.1)],
            PipeType::WE => [(pos.0, pos.1 + 1), (pos.0, pos.1 - 1)],
            PipeType::NE => [(pos.0, pos.1 + 1), (pos.0 - 1, pos.1)],
            PipeType::NW => [(pos.0, pos.1 - 1), (pos.0 - 1, pos.1)],
            PipeType::SE => [(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
            PipeType::SW => [(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)],
        }
    }
}

fn in_range(m: &Vec<Vec<char>>, s: (usize, usize)) -> bool {
    let (x, y) = s;
    x < m.len() && y < m[0].len()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    input.lines().enumerate().fold(
        (Vec::new(), (0_usize, 0_usize)),
        |(mut acc, mut s), (i, line)| {
            let row = line.chars().collect::<Vec<char>>();
            let j: Vec<usize> = line
                .chars()
                .enumerate()
                .filter(|(_, x)| x == &'S')
                .map(|(j, _)| j)
                .collect();
            if !j.is_empty() {
                s = (i, j[0]);
            }
            acc.push(row);
            (acc, s)
        },
    )
}

fn clockwise(
    m: &Vec<Vec<char>>,
    source: (usize, usize),
) -> HashMap<(usize, usize), Option<(usize, usize)>> {
    let mut visited: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();

    visited.insert(source, None);
    let mut current = source;

    let pipe = PipeType::from_char(&m[current.0][current.1]);
    let next = pipe.can_go(current);

    // pick one
    visited.insert(next[0], Some(source));
    current = next[0];

    loop {
        let pipe = PipeType::from_char(&m[current.0][current.1]);
        let next = pipe.can_go(current);

        match (
            visited.contains_key(&next[0]),
            visited.contains_key(&next[1]),
        ) {
            (true, true) => break,
            (true, false) => {
                visited.insert(next[1], Some(current));
                current = next[1];
            }
            (false, true) => {
                visited.insert(next[0], Some(current));
                current = next[0];
            }
            (false, false) => panic!(),
        }
    }
    visited
}

fn find_inside(
    m: &Vec<Vec<char>>,
    visited: &HashMap<(usize, usize), Option<(usize, usize)>>,
) -> HashSet<(usize, usize)> {
    let mut inside: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..m.len() {
        let mut is_in = false;
        for j in 0..m[0].len() {
            if visited.contains_key(&(i, j)) {
                if m[i][j] == '|' || m[i][j] == 'L' || m[i][j] == 'J' {
                    is_in = !is_in;
                }
                continue;
            }

            if is_in {
                inside.insert((i, j));
            }
        }
    }
    inside
}

pub fn part1(input: &str) {
    // 6733
    let (mut m, source) = parse_input(input);
    m[source.0][source.1] = 'J'; // :)

    let visited = clockwise(&m, source);
    let ans = visited.len() / 2;
    println!("{:?}", ans);
}

pub fn part2(input: &str) {
    // 435
    let (mut m, source) = parse_input(input);
    m[source.0][source.1] = 'J'; // :)
    let visited = clockwise(&m, source);

    let inside = find_inside(&m, &visited);
    println!("{}", inside.len());
}
