fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(
            line.split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        );
        acc
    })
}

fn diffs(vector: &[i32]) -> Vec<i32> {
    let mut diff: Vec<i32> = Vec::new();
    for i in 1..vector.len() {
        let d = vector[i] - vector[i - 1];
        diff.push(d);
    }
    diff
}

fn next(vector: &[i32]) -> i32 {
    if vector.iter().all(|x| x == &0) {
        0
    } else {
        vector.last().unwrap() + next(&diffs(vector))
    }
}

fn prev(vector: &[i32]) -> i32 {
    if vector.iter().all(|x| x == &0) {
        0
    } else {
        vector[0] - prev(&diffs(vector))
    }
}

pub fn part1(input: &str) {
    // 2038472161
    let data = parse_input(input);
    let ans: i32 = data.iter().map(|v| next(v)).sum();
    println!("{ans}");
}

pub fn part2(input: &str) {
    // 1091
    let data = parse_input(input);
    let ans: i32 = data.iter().map(|v| prev(v)).sum();
    println!("{ans}");
}
