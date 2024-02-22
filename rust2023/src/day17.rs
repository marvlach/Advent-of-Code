use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn can_go(&self, (x, y): (usize, usize), n_rows: usize, n_cols: usize) -> bool {
        match self {
            Direction::Right => y < n_cols - 1,
            Direction::Left => y > 0,
            Direction::Up => x > 0,
            Direction::Down => x < n_rows - 1,
        }
    }

    fn next_pos(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Right => (x, y + 1),
            Direction::Left => (x, y - 1),
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
        }
    }
}

#[derive(Debug)]
struct Map {
    nodes: HashMap<(usize, usize), u32>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    fn get_val(&self, pos: (usize, usize)) -> u32 {
        *self.nodes.get(&pos).unwrap()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    pos: (usize, usize),
    dir: Direction,
    streak: u32,
    // dist: u32,
}

impl State {
    fn get_possible_next_state(&self, map: &Map, min_streak: u32, max_streak: u32) -> Vec<State> {
        let next_direction_streak_pairs = if min_streak > 0 && self.streak < min_streak - 1 {
            vec![(self.dir, self.streak + 1)]
        } else if self.streak == max_streak - 1 {
            match self.dir {
                Direction::Right | Direction::Left => {
                    vec![(Direction::Up, 0), (Direction::Down, 0)]
                }
                Direction::Up | Direction::Down => {
                    vec![(Direction::Right, 0), (Direction::Left, 0)]
                }
            }
        } else {
            vec![
                Direction::Right,
                Direction::Left,
                Direction::Up,
                Direction::Down,
            ]
            .iter()
            .map(|x| {
                if self.dir == *x {
                    (*x, self.streak + 1)
                } else {
                    (*x, 0)
                }
            })
            .collect()
        };
        next_direction_streak_pairs
            .iter()
            .filter(|(d, _)| d.can_go(self.pos, map.n_rows, map.n_cols))
            .map(|(d, s)| State {
                pos: d.next_pos(self.pos),
                dir: *d,
                streak: *s,
            })
            .collect()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct StateDist {
    state: State,
    dist: u32,
}
impl Ord for StateDist {
    fn cmp(&self, other: &Self) -> Ordering {
        // (self.value, &self.name).cmp(&(other.value, &other.name))
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for StateDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Map {
    let mut v = HashMap::new();
    let mut n_rows = 0;
    let mut n_cols = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.char_indices() {
            v.insert((i, j), c.to_digit(10).unwrap());
            n_cols = i;
        }
        n_rows = i;
    }
    Map {
        nodes: v,
        n_rows: n_rows + 1,
        n_cols: n_cols + 1,
    }
}

fn dijkstra(map: &Map, min_streak: u32, max_streak: u32, target: (usize, usize)) -> u32 {
    let mut dist_map: HashMap<State, u32> = HashMap::new();
    let mut priority_queue: BinaryHeap<StateDist> = BinaryHeap::new();
    let right = State {
        pos: (0, 0),
        dir: Direction::Right,
        streak: 0,
    };
    let down = State {
        pos: (0, 0),
        dir: Direction::Down,
        streak: 0,
    };
    priority_queue.push(StateDist {
        state: right,
        dist: 0,
    });
    dist_map.insert(right, 0);
    priority_queue.push(StateDist {
        state: down,
        dist: 0,
    });
    dist_map.insert(down, 0);

    while let Some(StateDist { state, dist }) = priority_queue.pop() {
        for next_state in state.get_possible_next_state(map, min_streak, max_streak) {
            let next_dist = dist + map.get_val(next_state.pos);
            if next_dist < *dist_map.get(&next_state).unwrap_or(&u32::MAX) {
                priority_queue.push(StateDist {
                    state: next_state,
                    dist: next_dist,
                });
                dist_map.insert(next_state, next_dist);
            }
        }
    }
    *dist_map
        .iter()
        .filter(|(k, _)| k.pos == target)
        .map(|(_, v)| v)
        .min()
        .unwrap()
}

pub fn part1(input: &str) {
    // 1260
    let map = parse_input(input);
    let target = (map.n_rows - 1, map.n_cols - 1);
    let ans = dijkstra(&map, 0, 3, target);
    println!("{:?}", ans);
}

pub fn part2(input: &str) {
    // 1416
    let map = parse_input(input);
    let target = (map.n_rows - 1, map.n_cols - 1);
    let ans = dijkstra(&map, 4, 10, target);
    println!("{:?}", ans);
}
