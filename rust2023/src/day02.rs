#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_input(input: &str) -> Vec<Vec<Bag>> {
    let mut games: Vec<Vec<Bag>> = Vec::new();
    for game in input.lines() {
        let mut rounds: Vec<Bag> = Vec::new();
        for picks in game.split(":").nth(1).unwrap().split(";") {
            let (mut red, mut green, mut blue): (u32, u32, u32) = (0, 0, 0);
            for balls in picks.split(",") {
                let mut a = balls.strip_prefix(" ").unwrap().split(" ");
                let num = a.next().unwrap().parse::<u32>().unwrap();
                match a.next().unwrap() {
                    "red" => {
                        red += num;
                    }
                    "green" => {
                        green += num;
                    }
                    "blue" => {
                        blue += num;
                    }
                    _ => panic!(),
                };
            }
            rounds.push(Bag { red, green, blue });
        }
        games.push(rounds);
    }
    games
}

fn game_is_possible(game: &Vec<Bag>, bag: &Bag) -> bool {
    for round in game {
        if round.red > bag.red || round.green > bag.green || round.blue > bag.blue {
            return false;
        }
    }
    true
}

fn power(game: &Vec<Bag>) -> u32 {
    let (mut max_red, mut max_green, mut max_blue): (u32, u32, u32) = (0, 0, 0);
    for round in game {
        max_red = std::cmp::max(max_red, round.red);
        max_green = std::cmp::max(max_green, round.green);
        max_blue = std::cmp::max(max_blue, round.blue);
    }
    max_red * max_green * max_blue
}

fn count_possible(games: &Vec<Vec<Bag>>, bag: &Bag) -> u32 {
    let mut s: u32 = 0;
    for (id, game) in games.iter().enumerate() {
        if game_is_possible(game, &bag) {
            s += (id + 1) as u32;
        }
    }
    s
}

pub fn part1(input: &str) {
    let games = parse_input(input);
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let ans = count_possible(&games, &bag);
    println!("{}", ans);
}

pub fn part2(input: &str) {
    let games = parse_input(input);
    let ans: u32 = games.iter().map(power).sum();
    println!("{}", ans);
}
