use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    mine: HashSet<u32>,
}

impl Card {
    fn count_matches(&self) -> usize {
        self.winning.intersection(&self.mine).count()
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut v: Vec<Card> = Vec::new();
    for line in input.lines() {
        let mut a = line.split(&[':', '|'][..]);
        a.next();
        let mut winning: HashSet<u32> = HashSet::new();
        let mut mine: HashSet<u32> = HashSet::new();
        for i in a.next().unwrap().split(' ').filter(|x| !x.is_empty()) {
            winning.insert(i.parse::<u32>().unwrap());
        }
        for i in a.next().unwrap().split(' ').filter(|x| !x.is_empty()) {
            mine.insert(i.parse::<u32>().unwrap());
        }
        v.push(Card { winning, mine });
    }
    v
}

pub fn part1(input: &str) {
    let cards = parse_input(input);
    let ans: u32 = cards
        .iter()
        .map(|x| {
            let count = x.count_matches() as u32;
            if count > 0 {
                2_u32.pow(count - 1)
            } else {
                0
            }
        })
        .sum();
    println!("{ans}");
}

pub fn part2(input: &str) {
    let cards = parse_input(input);

    let mut card_counts = HashMap::new();
    for c in 0..cards.len() {
        card_counts.insert(c, 1_usize);
    }

    for (card_i, card) in cards.iter().enumerate() {
        let card_copies = *card_counts.get(&card_i).unwrap();
        let count_matches = card.count_matches();
        for offset in 0..count_matches {
            *card_counts.get_mut(&(card_i + 1 + offset)).unwrap() += card_copies;
        }
    }
    let ans: usize = card_counts.values().sum();
    println!("{ans}");
}
