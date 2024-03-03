use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::str::Lines;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum RuleResult {
    A,
    R,
    M(String),
}

impl RuleResult {
    fn from(s: &str) -> RuleResult {
        match s {
            "A" => RuleResult::A,
            "R" => RuleResult::R,
            _ => RuleResult::M(String::from(s)),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Rating {
    X,
    M,
    A,
    S,
}

impl Rating {
    fn from(c: char) -> Rating {
        match c {
            'x' => Rating::X,
            'm' => Rating::M,
            'a' => Rating::A,
            's' => Rating::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Range {
    from: u64, // closed
    to: u64,   // open
}

impl Range {
    fn from_expression(exp: &str) -> Range {
        let op = exp[1..2].chars().last().unwrap();
        let right: u64 = exp[2..].parse().unwrap();
        match op {
            '>' => Range {
                from: right + 1,
                to: 4001,
            },
            '<' => Range { from: 1, to: right },
            _ => panic!(),
        }
    }

    fn count_points(&self) -> u64 {
        self.to - self.from
    }

    fn compare(&self, other: &Range) -> (Option<Range>, Option<Range>) {
        let other_is_upper = other.to == 4001;
        if other_is_upper {
            if self.from >= other.from {
                (Some(*self), None)
            } else if self.to <= other.from {
                (None, Some(*self))
            } else {
                (
                    Some(Range {
                        from: other.from,
                        to: self.to,
                    }),
                    Some(Range {
                        from: self.from,
                        to: other.from,
                    }),
                )
            }
        } else {
            if self.from >= other.to {
                (None, Some(*self))
            } else if self.to <= other.to {
                (Some(*self), None)
            } else {
                (
                    Some(Range {
                        from: self.from,
                        to: other.to,
                    }),
                    Some(Range {
                        from: other.to,
                        to: self.to,
                    }),
                )
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct HyperCube {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl HyperCube {
    fn volume(&self) -> u64 {
        self.x.count_points()
            * self.m.count_points()
            * self.a.count_points()
            * self.s.count_points()
    }
}

trait Rule: Debug {
    fn apply_to(&self, cube: &HyperCube) -> (Option<HyperCube>, Option<HyperCube>);
    fn apply(&self) -> &RuleResult;
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct ComparisonRule {
    left: Rating,
    range: Range,
    res: RuleResult,
}

impl Rule for ComparisonRule {
    fn apply_to(&self, cube: &HyperCube) -> (Option<HyperCube>, Option<HyperCube>) {
        match self.left {
            Rating::X => {
                let (new_x_in, new_x_out) = cube.x.compare(&self.range);
                let in_cube = if let Some(range) = new_x_in {
                    Some(HyperCube {
                        x: range,
                        m: cube.m,
                        a: cube.a,
                        s: cube.s,
                    })
                } else {
                    None
                };
                let out_cube = if let Some(range) = new_x_out {
                    Some(HyperCube {
                        x: range,
                        m: cube.m,
                        a: cube.a,
                        s: cube.s,
                    })
                } else {
                    None
                };
                (in_cube, out_cube)
            }
            Rating::M => {
                let (new_m_in, new_m_out) = cube.m.compare(&self.range);
                let in_cube = if let Some(range) = new_m_in {
                    Some(HyperCube {
                        x: cube.x,
                        m: range,
                        a: cube.a,
                        s: cube.s,
                    })
                } else {
                    None
                };
                let out_cube = if let Some(range) = new_m_out {
                    Some(HyperCube {
                        x: cube.x,
                        m: range,
                        a: cube.a,
                        s: cube.s,
                    })
                } else {
                    None
                };
                (in_cube, out_cube)
            }
            Rating::A => {
                let (new_a_in, new_a_out) = cube.a.compare(&self.range);
                let in_cube = if let Some(range) = new_a_in {
                    Some(HyperCube {
                        x: cube.x,
                        m: cube.m,
                        a: range,
                        s: cube.s,
                    })
                } else {
                    None
                };
                let out_cube = if let Some(range) = new_a_out {
                    Some(HyperCube {
                        x: cube.x,
                        m: cube.m,
                        a: range,
                        s: cube.s,
                    })
                } else {
                    None
                };
                (in_cube, out_cube)
            }
            Rating::S => {
                let (new_s_in, new_s_out) = cube.s.compare(&self.range);
                let in_cube = if let Some(range) = new_s_in {
                    Some(HyperCube {
                        x: cube.x,
                        m: cube.m,
                        a: cube.a,
                        s: range,
                    })
                } else {
                    None
                };
                let out_cube = if let Some(range) = new_s_out {
                    Some(HyperCube {
                        x: cube.x,
                        m: cube.m,
                        a: cube.a,
                        s: range,
                    })
                } else {
                    None
                };
                (in_cube, out_cube)
            }
        }
    }

    fn apply(&self) -> &RuleResult {
        &self.res
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct CatchRule {
    res: RuleResult,
}

impl Rule for CatchRule {
    fn apply_to(&self, cube: &HyperCube) -> (Option<HyperCube>, Option<HyperCube>) {
        (Some(*cube), None)
    }

    fn apply(&self) -> &RuleResult {
        &self.res
    }
}

#[derive(Debug)]
struct RuleSet {
    list: Vec<Box<dyn Rule>>,
}

impl RuleSet {
    fn get_next_for_part(&self, part: &HyperCube) -> Option<&RuleResult> {
        for rule in &self.list {
            let (inside, outside) = rule.apply_to(part);
            if let Some(cube) = inside {
                let next = rule.apply();
                return Some(next);
            }
        }
        panic!();
    }

    fn get_next_for_cube(&self, cube: &HyperCube) -> Vec<(HyperCube, &RuleResult)> {
        let mut result_cubes = Vec::new();
        let mut remainder = *cube;
        for rule in &self.list {
            let (inside, outside) = rule.apply_to(&remainder);
            if let Some(c) = inside {
                let next = rule.apply();
                result_cubes.push((c, next));
            }
            if let Some(c) = outside {
                remainder = c;
            }
        }
        result_cubes
    }
}

fn rule_factory(r: &str) -> Box<dyn Rule> {
    if r.contains(":") {
        let mut splitted = r.split(":");
        let expression = splitted.next().unwrap();
        let res = RuleResult::from(splitted.next().unwrap());
        let left = Rating::from(expression[0..1].chars().last().unwrap());
        let range: Range = Range::from_expression(expression);
        Box::new(ComparisonRule { left, range, res })
    } else {
        Box::new(CatchRule {
            res: RuleResult::from(r),
        })
    }
}
fn parse_workflows(lines: &mut Lines, workflows: &mut HashMap<String, RuleSet>) {
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let mut chunks = line.split("{");
        let name = String::from(chunks.next().unwrap());
        let rest = chunks.next().unwrap().replace("}", "");
        let rules = rest.split(",").map(|r| rule_factory(r)).collect();
        let ruleset = RuleSet { list: rules };
        workflows.insert(name, ruleset);
    }
}

fn parse_parts(lines: &mut Lines, parts: &mut Vec<HyperCube>) {
    while let Some(line) = lines.next() {
        let no = line.replace("{", "").replace("}", "");
        let mut line = no.split(",");
        let x = line.next().unwrap()[2..].parse().unwrap();
        let m = line.next().unwrap()[2..].parse().unwrap();
        let a = line.next().unwrap()[2..].parse().unwrap();
        let s = line.next().unwrap()[2..].parse().unwrap();
        let part = HyperCube {
            x: Range { from: x, to: x },
            m: Range { from: m, to: m },
            a: Range { from: a, to: a },
            s: Range { from: s, to: s },
        };
        parts.push(part);
    }
}
fn parse_input(input: &str) -> (HashMap<String, RuleSet>, Vec<HyperCube>) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut lines = input.lines();
    parse_workflows(&mut lines, &mut workflows);
    parse_parts(&mut lines, &mut parts);
    (workflows, parts)
}

fn run_for_parts(workflows: HashMap<String, RuleSet>, parts: Vec<HyperCube>) -> u64 {
    let mut s = 0;
    for part in parts {
        let mut curr_worflow = workflows.get("in").unwrap();
        loop {
            let process_result = curr_worflow.get_next_for_part(&part).unwrap();
            match process_result {
                RuleResult::A => {
                    s += part.x.from + part.m.from + part.a.from + part.s.from;
                    break;
                }
                RuleResult::R => {
                    break;
                }
                RuleResult::M(s) => {
                    curr_worflow = workflows.get(s).unwrap();
                }
            }
        }
    }
    s
}

fn run_for_cube(workflows: HashMap<String, RuleSet>, cube: HyperCube) -> u64 {
    let mut s = 0;
    let mut queue: VecDeque<(HyperCube, &RuleResult)> = VecDeque::new();
    let init_workflow = RuleResult::M(String::from("in"));
    queue.push_back((cube, &init_workflow));

    while let Some((c, dest)) = queue.pop_front() {
        match dest {
            RuleResult::A => s += c.volume(),
            RuleResult::R => {}
            RuleResult::M(d) => {
                let curr_worflow = workflows.get(d).unwrap();
                for (cc, rr) in curr_worflow.get_next_for_cube(&c) {
                    queue.push_back((cc, rr))
                }
            }
        }
    }

    s
}

pub fn part1(input: &str) {
    // 386787
    let (workflows, parts) = parse_input(input);
    let ans = run_for_parts(workflows, parts);
    println!("{ans}");
}

pub fn part2(input: &str) {
    // 131029523269531
    let (workflows, _) = parse_input(input);
    let ans = run_for_cube(
        workflows,
        HyperCube {
            x: Range { from: 1, to: 4001 },
            m: Range { from: 1, to: 4001 },
            a: Range { from: 1, to: 4001 },
            s: Range { from: 1, to: 4001 },
        },
    );
    println!("{ans}");
}
