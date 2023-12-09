use std::{cmp::Ordering, collections::HashMap, iter::zip};
use Ordering::{Equal, Greater, Less};

#[derive(Debug, Eq, PartialEq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid character for card"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}
use HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.partial_cmp(&other.hand_type).unwrap()
    }
}

fn tie_break_based_on_card_order(
    a: &[Card; 5],
    b: &[Card; 5],
    card_order: &[Card; 13],
) -> Ordering {
    for (c_a, c_b) in zip(a, b) {
        let index_a = card_order.iter().position(|r| r == c_a).unwrap();
        let index_b = card_order.iter().position(|r| r == c_b).unwrap();
        if index_a > index_b {
            return Greater;
        }
        if index_a < index_b {
            return Less;
        }
    }
    Equal
}

fn parse_input(
    input: &str,
    get_hand_type: for<'a> fn(HashMap<&'a Card, i32>) -> HandType,
) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut i = line.split(" ");
        let cards = i
            .next()
            .unwrap()
            .chars()
            .map(|ch| Card::from_char(ch))
            .collect::<Vec<Card>>();
        let cards: [Card; 5] = cards.try_into().unwrap();
        let bid = i.next().unwrap().parse::<u32>().unwrap();
        let frequencies: HashMap<&Card, i32> = cards.iter().fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        });

        let hand_type = get_hand_type(frequencies);
        hands.push(Hand {
            cards,
            bid,
            hand_type,
        })
    }
    hands
}

fn get_hand_type_1(f: HashMap<&Card, i32>) -> HandType {
    match &f.len() {
        1 => FiveOfKind,
        2 => {
            if f.values().map(|v| v).any(|v| *v == 4) {
                FourOfKind
            } else {
                FullHouse
            }
        }
        3 => {
            if f.values().map(|v| v).any(|v| *v == 3) {
                ThreeOfKind
            } else {
                TwoPair
            }
        }
        4 => OnePair,
        5 => HighCard,
        _ => panic!(),
    }
}

fn get_hand_type_2(f: HashMap<&Card, i32>) -> HandType {
    let joker_count = *f.get(&Card::Joker).unwrap_or(&0);
    if joker_count == 0 | 5 {
        return get_hand_type_1(f);
    }

    let mut max_potential = HighCard;
    for (no_joker, _) in f.iter().filter(|(k, _)| k != &&&Card::Joker) {
        let mut potential_frequencies = f.iter().filter(|(k, _)| k != &&&Card::Joker).fold(
            HashMap::new(),
            |mut map, (key, val)| {
                map.insert(*key, *val);
                map
            },
        );
        potential_frequencies
            .entry(*no_joker)
            .and_modify(|frq| *frq += joker_count);
        let potential_hand_type = get_hand_type_1(potential_frequencies);

        if potential_hand_type > max_potential {
            max_potential = potential_hand_type
        }
    }
    max_potential
}

fn run(
    input: &str,
    hand_order: &[Card; 13],
    get_hand_type: for<'a> fn(HashMap<&'a Card, i32>) -> HandType,
) -> u32 {
    let mut hands = parse_input(input, get_hand_type);
    hands.sort_by(|a, b| match a.cmp(b) {
        Less => Less,
        Equal => tie_break_based_on_card_order(&a.cards, &b.cards, hand_order),
        Greater => Greater,
    });
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as u32) * hand.bid)
        .sum()
}
pub fn part1(input: &str) {
    // 246912307
    let hand_order = [
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Joker,
        Card::Queen,
        Card::King,
        Card::Ace,
    ];
    let ans = run(input, &hand_order, get_hand_type_1);
    println!("{ans}");
}

pub fn part2(input: &str) {
    // 246894760
    let hand_order = [
        Card::Joker,
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Queen,
        Card::King,
        Card::Ace,
    ];
    let ans = run(input, &hand_order, get_hand_type_2);
    println!("{ans}");
}
