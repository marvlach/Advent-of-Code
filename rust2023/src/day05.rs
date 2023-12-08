#[derive(Debug)]
struct Rule {
    source: Range,
    dest: Range,
}

#[derive(Debug)]
enum RuleRange {
    PartialLeft {
        left: Range,
        right: Range,
    },
    PartialRight {
        left: Range,
        right: Range,
    },
    Inside {
        middle: Range,
    },
    Outside {
        left: Range,
        middle: Range,
        right: Range,
    },
    CompleteLeft {
        left: Range,
    },
    CompleteRight {
        right: Range,
    },
}

impl Rule {
    fn split_range_2(&self, range: &Range) -> RuleRange {
        // partial left
        if range.0 < self.source.0 && range.1 >= self.source.0 && range.1 <= self.source.1 {
            return RuleRange::PartialLeft {
                left: Range(range.0, self.source.0),
                right: self.apply_to_range(&Range(self.source.0, range.1)),
            };
        }
        // partial right
        if range.1 > self.source.1 && range.0 < self.source.1 && range.0 >= self.source.0 {
            return RuleRange::PartialRight {
                left: self.apply_to_range(&Range(range.0, self.source.1)),
                right: Range(self.source.1, range.1),
            };
        }
        // in
        if range.0 >= self.source.0
            && range.0 < self.source.1
            && range.1 > self.source.0
            && range.1 <= self.source.1
        {
            return RuleRange::Inside {
                middle: self.apply_to_range(&Range(range.0, range.1)),
            };
        }
        // out
        if range.0 < self.source.0 && range.1 > self.source.1 {
            return RuleRange::Outside {
                left: Range(range.0, self.source.0),
                middle: self.apply_to_range(&Range(self.source.0, self.source.1)),
                right: Range(self.source.1, range.1),
            };
        }
        // to the left
        if range.1 < self.source.0 {
            return RuleRange::CompleteLeft {
                left: Range(range.0, range.1),
            };
        }
        // to the right
        if range.0 >= self.source.1 {
            return RuleRange::CompleteRight {
                right: Range(range.0, range.1),
            };
        }

        panic!(
            "Point where something went wrong. Rule={:?}, Range={:?}",
            self, range
        );
    }

    fn apply_to_range(&self, range: &Range) -> Range {
        Range(
            range.0 + self.dest.0 - self.source.0,
            range.1 + self.dest.1 - self.source.1,
        )
    }
}

#[derive(Debug, PartialEq)]
struct Range(u128, u128);

impl Range {
    fn apply_rules(&self, rules: &[Rule]) -> Vec<Range> {
        let mut range_converted: Vec<Range> = Vec::new();
        let mut running_range = Range(self.0, self.1);
        for rule in rules {
            match rule.split_range_2(&running_range) {
                RuleRange::PartialLeft { left, right } => {
                    range_converted.push(left);
                    range_converted.push(right);
                    return range_converted;
                }
                RuleRange::PartialRight { left, right } => {
                    range_converted.push(left);
                    running_range = right;
                }
                RuleRange::Inside { middle } => {
                    range_converted.push(middle);
                    return range_converted;
                }
                RuleRange::Outside {
                    left,
                    middle,
                    right,
                } => {
                    range_converted.push(left);
                    range_converted.push(middle);
                    running_range = right;
                }
                RuleRange::CompleteLeft { left } => {
                    range_converted.push(left);
                    return range_converted;
                }
                RuleRange::CompleteRight { right } => {
                    running_range = right;
                }
            }
        }
        range_converted.push(running_range);
        range_converted
    }
}

#[derive(Debug)]
struct Mapping {
    rules: Vec<Rule>,
}

fn parse_input_mappings(input: &str) -> Vec<Mapping> {
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut mapping: Vec<Rule> = Vec::new();
    for line in input.lines().filter(|l| !l.is_empty()).skip(1) {
        if line.contains("map") {
            mapping.sort_by_key(|r| r.source.0);
            mappings.push(Mapping { rules: mapping });
            mapping = Vec::new();
        } else {
            let nums: Vec<u128> = line
                .split(" ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect();
            // dest, source, count
            let dest = *nums.get(0).unwrap();
            let source = *nums.get(1).unwrap();
            let count = *nums.get(2).unwrap();
            mapping.push(Rule {
                source: Range(source, source + count),
                dest: Range(dest, dest + count),
            })
        }
    }
    mapping.sort_by_key(|r| r.source.0);
    mappings.push(Mapping { rules: mapping });
    mappings.remove(0);
    mappings
}

fn parse_input_seeds_1(input: &str) -> Vec<Range> {
    input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|x| x.parse::<u128>().unwrap())
        .map(|x| Range(x, x + 1))
        .collect::<Vec<Range>>()
}

fn parse_input_seeds_2(input: &str) -> Vec<Range> {
    let nums = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    let mut ranges: Vec<Range> = Vec::new();
    for i in (0..nums.len()).step_by(2) {
        let start = *nums.get(i).unwrap();
        let end = *nums.get(i + 1).unwrap() + start;
        ranges.push(Range(start, end));
    }
    ranges
}

fn run(ranges: Vec<Range>, mappings: Vec<Mapping>) -> u128 {
    let mut locs: Vec<u128> = Vec::new();

    for range in ranges {
        let mut before: Vec<Range> = vec![range];

        for mapping in mappings.iter() {
            before = before
                .iter()
                .flat_map(|x| x.apply_rules(&mapping.rules))
                .collect::<Vec<Range>>();
        }
        locs.extend(before.iter().map(|r| r.0));
    }
    *locs.iter().min().unwrap()
}

pub fn part1(input: &str) {
    //910845529
    let ranges = parse_input_seeds_1(input);
    let mappings = parse_input_mappings(input);

    let ans = run(ranges, mappings);
    println!("Ans: {ans}");
}

pub fn part2(input: &str) {
    //77435348
    let ranges = parse_input_seeds_2(input);
    let mappings = parse_input_mappings(input);

    let ans = run(ranges, mappings);
    println!("Ans: {ans}");
}
