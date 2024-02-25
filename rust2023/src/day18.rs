fn parse_input(input: &str, parse_line: fn(&str) -> (&str, i64)) -> (Vec<(i64, i64)>, i64) {
    let mut points = Vec::new();
    let mut curr = (0, 0);
    points.push(curr);
    let mut n_boundary = 0;
    for line in input.lines() {
        let (direction, dist) = parse_line(line);
        let next_point = match direction {
            "R" => (curr.0 + dist, curr.1),
            "L" => (curr.0 - dist, curr.1),
            "D" => (curr.0, curr.1 - dist),
            "U" => (curr.0, curr.1 + dist),
            _ => panic!(),
        };
        n_boundary += dist;
        points.push(next_point);
        curr = next_point;
    }
    (points, n_boundary)
}

fn shoelace(points: &Vec<(i64, i64)>) -> i64 {
    let mut s = 0;
    for p in 1..points.len() {
        let (a, c) = points[p - 1];
        let (b, d) = points[p];
        s += a * d - b * c;
    }
    s.abs()
}

fn picks_theorem(area: i64, n_boundary: i64) -> i64 {
    (2 * area + 2 - n_boundary) / 2
}

pub fn part1(input: &str) {
    // 62573
    fn parse_line(line: &str) -> (&str, i64) {
        let mut chunks = line.split(" ");
        let direction = chunks.next().unwrap();
        let dist = chunks.next().unwrap().parse::<i64>().unwrap();
        (direction, dist)
    }

    let (points, n_boundary) = parse_input(input, parse_line);
    let area = shoelace(&points) / 2;
    let n_internal = picks_theorem(area, n_boundary);
    let ans = n_boundary + n_internal;
    println!("{ans}");
}

pub fn part2(input: &str) {
    // 54662804037719
    fn parse_line(line: &str) -> (&str, i64) {
        let mut chunks = line.split(" ");
        chunks.next();
        chunks.next();

        let hex = &chunks.next().unwrap()[2..8];
        let dist = i64::from_str_radix(&hex[0..5], 16).unwrap();
        let direction = match &hex[5..6] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!(),
        };

        (direction, dist)
    }
    let (points, n_boundary) = parse_input(input, parse_line);
    let area = shoelace(&points) / 2;
    let n_internal = picks_theorem(area, n_boundary);
    let ans = n_boundary + n_internal;
    println!("{ans}");
}
