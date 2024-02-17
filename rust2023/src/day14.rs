
use crate::utils::rotate_clockwise;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut rocks = vec![];
    for line in input.lines() {
        let v: Vec<char> = line.chars().collect();
        rocks.push(v);
    }
    rocks
}

fn roll_north(rocks: &mut Vec<Vec<char>>) {
    let n_rows = rocks.len();
    let n_cols = rocks[0].len();

    for i in 1..n_rows {
        for j in 0..n_cols {
            if rocks[i][j] != 'O' {
                continue;
            }

            let mut new_i: usize = 0;
            for k in (0..i).rev() {
                if rocks[k][j] != '.' {
                    new_i = k + 1;
                    break;
                }
            }
            rocks[i][j] = '.';
            rocks[new_i][j] = 'O';
        }
    }
}

fn calc_load(rocks: &Vec<Vec<char>>) -> usize {
    let mut s = 0;
    for i in 0..rocks.len() {
        let row_count = rocks[i].iter().filter(|x| x == &&'O').count();
        s += row_count * (rocks.len() - i)
    }
    s
}

fn rocks_to_hash(rocks: &Vec<Vec<char>>) -> u64 {
    let hash_vec: Vec<u128> = rocks
        .iter()
        .map(|r| {
            u128::from_str_radix(
                &r.iter()
                    .collect::<String>()
                    .replace(".", "0")
                    .replace("#", "0")
                    .replace("O", "1"),
                2,
            )
            .unwrap()
        })
        .collect();
    let mut hasher = DefaultHasher::new();
    hash_vec.hash(&mut hasher);
    hasher.finish()
}

fn complete_round(mut rocks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    roll_north(&mut rocks);
    rocks = rotate_clockwise(rocks);
    roll_north(&mut rocks);
    rocks = rotate_clockwise(rocks);
    roll_north(&mut rocks);
    rocks = rotate_clockwise(rocks);
    roll_north(&mut rocks);
    rotate_clockwise(rocks)
}

fn cycle_detection(mut rocks: Vec<Vec<char>>) -> (u32, u32, Vec<Vec<char>>) {
    let mut loads = HashMap::new();
    let mut i = 0;
    let hash_repeated = 'outer: loop {
        
        rocks = complete_round(rocks);

        let rocks_hash = rocks_to_hash(&rocks);

        if loads.contains_key(&rocks_hash) {
            break 'outer *loads.get(&rocks_hash).unwrap();
        }

        loads.insert(rocks_hash, i);
        i += 1;
    };
    (hash_repeated, i, rocks)
}

pub fn part1(input: &str) {
    // 110090
    let mut rocks = parse_input(input);
    roll_north(&mut rocks);
    let ans = calc_load(&rocks);
    println!("{ans}");
}

pub fn part2(input: &str) {
    // 95254
    let rocks = parse_input(input);
    let (first_occurence, second_occurence, mut rocks) = cycle_detection(rocks);
    let before = first_occurence;
    let period = second_occurence - first_occurence;
    let remaining = (1_000_000_000 - before) % period - 1;
    for _ in 0..remaining {
        rocks = complete_round(rocks);
    }
    let ans = calc_load(&rocks);
    println!("{ans}");
}
