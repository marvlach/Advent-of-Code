use cached::proc_macro::cached;
use cached::UnboundCache;

#[derive(Debug)]
struct Row {
    springs: String,
    broken: Vec<u64>,
}

fn parse_input(input: &str, part: u8) -> Vec<Row> {
    let mut res = vec![];
    for line in input.lines() {
        let mut liter = line.split(" ");
        let mut springs: String = liter.next().unwrap().chars().collect();
        if part == 2 {
            springs.insert(springs.len(), '?');
            springs = springs.repeat(5);
            springs.remove(springs.len() - 1);
        }
        springs.insert(0, '.');
        springs.insert(springs.len(), '.');

        let mut broken: Vec<u64> = liter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        if part == 2 {
            broken = broken.repeat(5);
        }
        res.push(Row { springs, broken })
    }
    res
}

fn is_legit(s: &str, broken: &[u64]) -> bool {
    let mut counting = false;
    let mut count = 0;
    let mut i: usize = match broken.iter().position(|&x| x != 0) {
        Some(i) => i,
        None => {
            return !s.contains("#");
        }
    };

    for c in s.chars() {
        if counting && c == '#' {
            count += 1;
        } else if counting && c == '.' {
            if i >= broken.len() || count != broken[i] {
                return false;
            }
            count = 0;
            i += 1;
            counting = false;
        } else if c == '#' {
            counting = true;
            count += 1;
        }
    }
    i == broken.len()
}

#[cached(
    type = "UnboundCache<String, u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{:?}{:?}{}", s, broken, currently_counting) }"#
)]
fn count_ways_fast_memoized(s: &str, broken: &[u64], currently_counting: bool) -> u64 {
    if currently_counting && broken[0] == 0 && s.starts_with("#") {
        return 0;
    }

    if !s.contains("?") {
        return if is_legit(&s, &broken) { 1 } else { 0 };
    }

    if broken.is_empty() {
        return if s.contains("#") { 0 } else { 1 };
    }

    if broken[0] == 0 {
        return match s.starts_with('#') {
            true => 0,
            false => count_ways_fast_memoized(&s[1..], &broken[1..], false),
        };
    }
    if s.starts_with(".") {
        if currently_counting {
            return 0;
        }
        return count_ways_fast_memoized(&s[1..], broken, false);
    }

    let new_broken: Vec<u64> = [broken[0] - 1]
        .iter()
        .chain(&broken[1..])
        .map(|&x| x)
        .collect();

    if s.starts_with("#") {
        let fi = broken[0];
        let mut a = vec![fi - 1];
        a.extend(broken[1..].iter());
        return count_ways_fast_memoized(&s[1..], &new_broken, true);
    }

    if currently_counting {
        return count_ways_fast_memoized(&s[1..], &new_broken, true);
    }
    return count_ways_fast_memoized(&s[1..], broken, false)
        + count_ways_fast_memoized(&s[1..], &new_broken, true);
}

pub fn part1(input: &str) {
    // 7705
    let rows = parse_input(input, 1);
    let ans: u64 = rows
        .iter()
        .map(|r| count_ways_fast_memoized(&r.springs, &r.broken, false))
        .sum();
    println!("{ans}");
}

pub fn part2(input: &str) {
    // 50338344809230
    let rows = parse_input(input, 2);
    let ans: u64 = rows
        .iter()
        .map(|r| count_ways_fast_memoized(&r.springs, &r.broken, false))
        .sum();
    println!("{ans}");
}
