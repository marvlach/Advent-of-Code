#[derive(Debug, PartialEq, Eq, Hash)]
struct Galaxy(u64, u64);

fn parse_input(input: &str) -> (Vec<Galaxy>, u64, u64) {
    let mut galaxies = vec![];
    let mut n_rows: u64 = 0;
    let mut n_cols: u64 = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.char_indices() {
            if c == '#' {
                galaxies.push(Galaxy(i as u64, j as u64));
            }
            n_cols = j as u64;
        }
        n_rows = i as u64;
    }
    (galaxies, n_rows, n_cols)
}

fn expand(galaxies: Vec<Galaxy>, n_rows: u64, n_cols: u64, by: u64) -> Vec<Galaxy> {
    fn get_no_galaxies(galaxies: &[Galaxy], margin: u64, axis: u32) -> Vec<u64> {
        (0..margin)
            .collect::<Vec<u64>>()
            .into_iter()
            .filter(|n| {
                galaxies
                    .iter()
                    .all(|g| (g.0 != *n && axis == 0) || (g.1 != *n && axis == 1))
            })
            .collect::<Vec<u64>>()
    }

    fn get_moved_galaxy(g: Galaxy, axis: u32, no_galaxies: &[u64], by: u64) -> Galaxy {
        let mut positions_to_move: u64 = 0;
        for ng in no_galaxies {
            if (*ng > g.1 && axis == 1) || (*ng > g.0 && axis == 0) {
                break;
            }
            positions_to_move += by;
        }
        if axis == 0 {
            Galaxy(g.0 + positions_to_move, g.1)
        } else {
            Galaxy(g.0, g.1 + positions_to_move)
        }
    }
    let no_galaxy_rows = get_no_galaxies(&galaxies, n_rows, 0);
    let no_galaxy_cols = get_no_galaxies(&galaxies, n_cols, 1);
    galaxies
        .into_iter()
        .map(|g| get_moved_galaxy(g, 0, &no_galaxy_rows, by))
        .map(|g| get_moved_galaxy(g, 1, &no_galaxy_cols, by))
        .collect()
}

fn sum_dists(galaxies: &[Galaxy]) -> u64 {
    let mut s = 0;
    for (i, g_from) in galaxies.iter().enumerate() {
        for (j, g_to) in galaxies.iter().enumerate() {
            if i >= j {
                continue;
            }
            s += g_from.0.abs_diff(g_to.0) + (g_from.1.abs_diff(g_to.1));
        }
    }
    s
}

pub fn part1(input: &str) {
    // 9418609
    let (mut galaxies, n_rows, n_cols) = parse_input(input);
    galaxies = expand(galaxies, n_rows, n_cols, 1);
    let ans = sum_dists(&galaxies);
    println!("{:?}", ans);
}

pub fn part2(input: &str) {
    // 593821230983
    let (mut galaxies, n_rows, n_cols) = parse_input(input);
    galaxies = expand(galaxies, n_rows, n_cols, 999999);
    let ans = sum_dists(&galaxies);
    println!("{:?}", ans);
}
