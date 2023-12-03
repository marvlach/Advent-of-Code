#[derive(Debug)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    fn is_neighbour(&self, other: &Pos) -> bool {
        if self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1 {
            //println!("{:?} is neighbour to {:?}", self, other);
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Number {
    num: String,
    pos: Pos,
}

impl Number {
    fn neighbours_symbol(&self, symbol: &Symbol) -> bool {
        for (i, _d) in self.num.char_indices() {
            let digit_pos = Pos {
                x: self.pos.x,
                y: self.pos.y + (i as u32),
            };
            if digit_pos.is_neighbour(&symbol.pos) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    pos: Pos,
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut mid_num = false;
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut current_num = String::from("");
    let mut current_num_pos: Pos = Pos { x: 0, y: 0 };
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.char_indices() {
            match (c.is_digit(10), mid_num) {
                (true, true) => {
                    current_num += &c.to_string();
                }
                (true, false) => {
                    mid_num = true;
                    current_num += &c.to_string();
                    current_num_pos = Pos {
                        x: x as u32,
                        y: y as u32,
                    };
                }
                (false, true) => {
                    mid_num = false;
                    numbers.push(Number {
                        pos: current_num_pos,
                        num: current_num,
                    });
                    current_num = String::from("");
                    current_num_pos = Pos { x: 0, y: 0 };
                    if c != '.' {
                        symbols.push(Symbol {
                            symbol: c,
                            pos: Pos {
                                x: x as u32,
                                y: y as u32,
                            },
                        });
                    }
                }
                (false, false) => {
                    if c != '.' {
                        symbols.push(Symbol {
                            symbol: c,
                            pos: Pos {
                                x: x as u32,
                                y: y as u32,
                            },
                        });
                    }
                }
            }
        }
    }
    (numbers, symbols)
}

pub fn part1(input: &str) {
    let (numbers, symbols) = parse_input(input);
    let ans: u32 = numbers
        .into_iter()
        .filter(|n| symbols.iter().any(|s| n.neighbours_symbol(s)))
        .map(|n| n.num.parse::<u32>().unwrap())
        .sum();
    println!("{ans}")
}

pub fn part2(input: &str) {
    let (numbers, symbols) = parse_input(input);
    let ans: u32 = symbols
        .into_iter()
        .filter(|s| s.symbol == '*')
        .map(|s| {
            let mut neighbours = numbers.iter().filter(|n| n.neighbours_symbol(&s));
            let mut prod: u32 = 1;
            for i in 0..3 {
                match (i, neighbours.next()) {
                    (0 | 1, Some(Number { num, .. })) => prod *= num.parse::<u32>().unwrap(),
                    (0 | 1, None) => prod = 0,
                    (2, None) => (),
                    (_, _) => prod = 0,
                }
            }
            prod
        })
        .sum();
    println!("{ans}")
}
