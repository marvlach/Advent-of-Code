use crate::utils::transpose;

struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

fn to_binary(x: &Vec<char>) -> u32 {
    u32::from_str_radix(
        &x.iter()
            .collect::<String>()
            .replace(".", "0")
            .replace("#", "1"),
        2,
    )
    .expect("Not a binary number!")
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let mut res1: Vec<Pattern> = vec![];
    let mut pattern: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            let rows = pattern.iter().map(|x| to_binary(x)).collect();
            let cols = transpose(pattern).iter().map(|x| to_binary(x)).collect();
            res1.push(Pattern { rows, cols });

            pattern = vec![];
        } else {
            pattern.push(line.chars().collect());
        }
    }
    let rows = pattern.iter().map(|x| to_binary(x)).collect();
    let cols = transpose(pattern).iter().map(|x| to_binary(x)).collect();
    res1.push(Pattern { rows, cols });
    res1
}

fn do_it_1(p: &[u32]) -> usize {
    let n_rows = p.len();

    'counting_up: for mid in 1..n_rows {
        if p[mid - 1] != p[mid] {
            continue;
        }

        let mut count = (mid - 1 - 0).min(n_rows - 1 - mid);

        while count > 0 {
            if p[mid - 1 - count] != p[mid + count] {
                continue 'counting_up;
            }
            count -= 1;
        }
        return mid;
    }
    0
}

fn do_it_2(p: &[u32]) -> usize {
    let n_rows = p.len();

    'counting_up: for mid in 1..n_rows {
        let is_same = p[mid - 1] == p[mid];
        let one_bit_diff = p[mid - 1].abs_diff(p[mid]).is_power_of_two();

        if !is_same && !one_bit_diff {
            continue;
        }

        let mut swap_used = one_bit_diff;

        let mut count = (mid - 1 - 0).min(n_rows - 1 - mid);

        while count > 0 {
            let is_same = p[mid - 1 - count] == p[mid + count];
            let one_bit_diff = p[mid - 1 - count]
                .abs_diff(p[mid + count])
                .is_power_of_two();

            if (!is_same && !one_bit_diff) || (one_bit_diff && swap_used) {
                continue 'counting_up;
            }

            if one_bit_diff {
                swap_used = true;
            }

            count -= 1;
        }
        if swap_used {
            return mid;
        }
    }
    0
}

pub fn part1(input: &str) {
    // 34889
    let patterns = parse_input(input);
    let mut ans = 0;
    for p in patterns {
        let ind_row = do_it_1(&p.rows);
        let ind_col = do_it_1(&p.cols);
        ans += if ind_row != 0 { 100 * ind_row } else { ind_col }
    }
    //let ans = hor * 100 + ver;
    println!("{ans}");
}

pub fn part2(input: &str) {
    // 34224
    let patterns = parse_input(input);
    let mut ans = 0;
    for p in patterns {
        let ind_row = do_it_2(&p.rows);
        let ind_col = do_it_2(&p.cols);
        ans += if ind_row != 0 { 100 * ind_row } else { ind_col }
    }
    println!("{ans}");
}
