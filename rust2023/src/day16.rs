use std::{collections::HashSet, vec};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn can_go(&self, pos: (usize, usize), n_rows: usize, n_cols: usize) -> bool {
        match self {
            Direction::Right => pos.1 < n_cols - 1,
            Direction::Left => pos.1 > 0,
            Direction::Up => pos.0 > 0,
            Direction::Down => pos.0 < n_rows - 1,
        }
    }

    fn next_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Mirror {
    FrontSlash,
    BackSlash,
    Horizontal,
    Vertical,
    Empty,
}

impl Mirror {
    fn next_dir(&self, in_dir: Direction) -> Vec<Direction> {
        match (self, in_dir) {
            (Mirror::FrontSlash, Direction::Right) => vec![Direction::Up],
            (Mirror::FrontSlash, Direction::Left) => vec![Direction::Down],
            (Mirror::FrontSlash, Direction::Up) => vec![Direction::Right],
            (Mirror::FrontSlash, Direction::Down) => vec![Direction::Left],
            (Mirror::BackSlash, Direction::Right) => vec![Direction::Down],
            (Mirror::BackSlash, Direction::Left) => vec![Direction::Up],
            (Mirror::BackSlash, Direction::Up) => vec![Direction::Left],
            (Mirror::BackSlash, Direction::Down) => vec![Direction::Right],
            (Mirror::Horizontal, Direction::Right) => vec![Direction::Right],
            (Mirror::Horizontal, Direction::Left) => vec![Direction::Left],
            (Mirror::Horizontal, Direction::Up) => vec![Direction::Right, Direction::Left],
            (Mirror::Horizontal, Direction::Down) => vec![Direction::Right, Direction::Left],
            (Mirror::Vertical, Direction::Right) => vec![Direction::Up, Direction::Down],
            (Mirror::Vertical, Direction::Left) => vec![Direction::Up, Direction::Down],
            (Mirror::Vertical, Direction::Up) => vec![Direction::Up],
            (Mirror::Vertical, Direction::Down) => vec![Direction::Down],
            (Mirror::Empty, Direction::Right) => vec![Direction::Right],
            (Mirror::Empty, Direction::Left) => vec![Direction::Left],
            (Mirror::Empty, Direction::Up) => vec![Direction::Up],
            (Mirror::Empty, Direction::Down) => vec![Direction::Down],
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Mirror>> {
    let mut floor = vec![];
    for line in input.lines() {
        let v: Vec<Mirror> = line
            .chars()
            .map(|c| match c {
                '.' => Mirror::Empty,
                '/' => Mirror::FrontSlash,
                '\\' => Mirror::BackSlash,
                '|' => Mirror::Vertical,
                '-' => Mirror::Horizontal,
                _ => panic!(),
            })
            .collect();
        floor.push(v);
    }
    floor
}

fn get_next_positions(
    floor: &Vec<Vec<Mirror>>,
    pos: (usize, usize),
    dir: Direction,
) -> Vec<((usize, usize), Direction)> {
    floor[pos.0][pos.1]
        .next_dir(dir)
        .iter()
        .filter(|d| d.can_go(pos, floor.len(), floor[0].len()))
        .map(|d| (d.next_pos(pos), *d))
        .collect()
}

fn dfs(
    floor: &Vec<Vec<Mirror>>,
    pos: (usize, usize),
    dir: Direction,
    visited: &mut HashSet<((usize, usize), Direction)>,
) {
    let was_inserted = visited.insert((pos, dir));
    if !was_inserted {
        return;
    }
    for (next_pos, next_dir) in get_next_positions(floor, pos, dir) {
        dfs(floor, next_pos, next_dir, visited);
    }
}

fn run(floor: &Vec<Vec<Mirror>>, start_pos: (usize, usize), start_dir: Direction) -> usize {
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    dfs(&floor, start_pos, start_dir, &mut visited);
    visited
        .iter()
        .fold(HashSet::new(), |mut acc: HashSet<(usize, usize)>, x| {
            acc.insert(x.0);
            acc
        })
        .len()
}

pub fn part1(input: &str) {
    // 8249
    let floor = parse_input(input);
    let ans = run(&floor, (0, 0), Direction::Right);
    println!("{:?}", ans);
}

pub fn part2(input: &str) {
    // 8444
    let floor = parse_input(input);
    let mut starts: Vec<((usize, usize), Direction)> = Vec::new();
    for i in 0..floor.len() {
        starts.push(((i, 0), Direction::Right));
        starts.push(((i, floor[0].len() - 1), Direction::Left));
    }
    for j in 0..floor[0].len() {
        starts.push(((0, j), Direction::Down));
        starts.push(((floor.len() - 1, j), Direction::Up));
    }
    let ans = starts.iter().map(|s| run(&floor, s.0, s.1)).max().unwrap();
    println!("{:?}", ans);
}
