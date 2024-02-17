enum Operation {
    Minus,
    Equal(u32),
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .split(",")
        .map(|s| String::from(s))
        .fold(vec![], |mut acc: Vec<String>, s| {
            acc.push(s);
            acc
        })
}

fn to_hash(s: &str) -> u32 {
    s.chars().fold(0, |mut val, c| {
        val += c as u32;
        val *= 17;
        val %= 256;
        val
    })
}

fn get_label(s: &str) -> &str {
    if s.contains("-") {
        &s[0..s.len() - 1]
    } else {
        &s[0..s.len() - 2]
    }
}

fn get_op(s: &str) -> Operation {
    if s.contains("-") {
        Operation::Minus
    } else {
        Operation::Equal(s.chars().nth(s.len() - 1).unwrap().to_digit(10).unwrap())
    }
}

fn focusing_power(h: &Vec<Vec<(&str, u32)>>) -> usize {
    h.iter().enumerate().fold(0_usize, |acc, (i, row)| {
        acc + row.iter().enumerate().fold(0, |inner_acc, (j, val)| {
            inner_acc + (i + 1) * (j + 1) * val.1 as usize
        })
    })
}

pub fn part1(input: &str) {
    // 520500
    let v = parse_input(input);
    let ans = v.iter().map(|s| to_hash(s)).sum::<u32>();
    println!("{ans}");
}

pub fn part2(input: &str) {
    // 213097
    let v = parse_input(input);

    let mut h: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    for s in v.iter() {
        let label = get_label(&s);
        let label_hash = to_hash(label);

        let vector = &mut h[label_hash as usize];
        let label_position_in_vector = vector.iter().position(|x| x.0 == label);

        match (get_op(&s), label_position_in_vector) {
            (Operation::Minus, None) => {}
            (Operation::Minus, Some(i)) => {
                vector.remove(i);
            }
            (Operation::Equal(lens), None) => vector.push((label, lens)),
            (Operation::Equal(lens), Some(i)) => vector[i] = (label, lens),
        }
    }
    let ans = focusing_power(&h);
    println!("{ans}");
}
