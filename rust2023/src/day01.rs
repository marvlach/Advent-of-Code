use std::collections::HashMap;

fn sum_first_last(
    input: &str,
    forward: fn(&Vec<char>) -> Vec<char>,
    backward: fn(&Vec<char>) -> Vec<char>,
) -> u32 {
    input
        .lines()
        .map(|x| {
            let char_vec: Vec<char> = x.chars().collect();
            let first = forward(&char_vec)
                .into_iter()
                .filter(|x| x.is_numeric())
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last = backward(&char_vec)
                .into_iter()
                .filter(|x| x.is_numeric())
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>()
}


fn replace_nums(x: &Vec<char>, backwards: bool) -> Vec<char> {
    let mapping = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut s: String = String::new();
    let the_iter: Box<dyn Iterator<Item = &char>> = if backwards {
        Box::new(x.iter().rev())
    } else {
        Box::new(x.iter())
    };
    for c in the_iter {
        s = s + &c.to_string();
        for (k, v) in &mapping {
            let to_replace = if backwards {
                k.chars().rev().collect::<String>()
            } else {
                String::from(*k)
            };
            s = s.replace(&to_replace, v);
        }
    }
    if backwards {
        s.chars().rev().collect::<Vec<char>>()
    } else {
        s.chars().collect::<Vec<char>>()
    }
}

pub fn part1(input: &str) {
    let ans: u32 = sum_first_last(
        input,
        |x: &Vec<char>| x.to_owned(),
        |x: &Vec<char>| x.to_owned(),
    );
    println!("{:?}", ans);
}

pub fn part2(input: &str) {
    let ans: u32 = sum_first_last(
        input,
        |x| replace_nums(x, false),
        |x: &Vec<char>| replace_nums(x, true),
    );
    println!("{:?}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let a = "nineight".chars().collect();

        assert_eq!("9ight".chars().collect::<Vec<_>>(), replace_nums(&a, false));
    }

    #[test]
    fn test_backward() {
        let a = "nineight".chars().collect();

        assert_eq!("nin8".chars().collect::<Vec<_>>(), replace_nums(&a, true));
    }
}
