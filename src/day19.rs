use std::fs;

#[derive(Debug, Clone)]
enum Response {
    NotApplied,
    Compare(char, char, u32, String),
    Accept,
    Reject,
    Forward(String),
}

#[derive(Debug)]
struct Rule {
    name: String,
    conditions: Vec<Condition>,
}

impl Rule {
    fn apply(&self, part: &Part) -> Response {
        for cnd in &self.conditions {
            let resp = cnd.apply(part);

            if !matches!(resp, Response::NotApplied) {
                return resp;
            }
        }

        Response::NotApplied
    }
}

#[derive(Debug)]
struct Condition {
    resp: Response,
}

impl Condition {
    fn from_str(str: &str) -> Condition {
        let resp = if let Some(idx) = str.find(':') {
            Response::Compare(
                str.chars().nth(0).unwrap(),
                str.chars().nth(1).unwrap(),
                str[2..idx].parse::<u32>().unwrap(),
                str[idx + 1..].to_owned(),
            )
        } else if str.chars().nth(0).unwrap() == 'A' {
            Response::Accept
        } else if str.chars().nth(0).unwrap() == 'R' {
            Response::Reject
        } else {
            Response::Forward(str.to_owned())
        };

        Condition { resp }
    }

    fn apply(&self, part: &Part) -> Response {
        if let Response::Compare(ch, op, val, forward) = &self.resp {
            let part_val = match ch {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => panic!("Unknown ch {}", ch),
            };

            let matches = match op {
                '<' => part_val < *val,
                '>' => part_val > *val,
                _ => panic!("Unknown op {}", op),
            };

            if !matches {
                Response::NotApplied
            } else {
                match forward.as_str() {
                    "R" => Response::Reject,
                    "A" => Response::Accept,
                    _ => Response::Forward(forward.to_owned()),
                }
            }
        } else {
            self.resp.clone()
        }
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

pub fn solve(suffix: &str) -> anyhow::Result<()> {
    let input = fs::read_to_string(format!("input/d19{}", suffix))?;

    let mut rules = Vec::new();
    let mut parts = Vec::new();

    let mut itr = input.lines();
    while let Some(line) = itr.next() {
        if line == "" {
            break;
        }

        let (name, conditions) = line.split_once('{').unwrap();
        let conditions = conditions.replace("}", "");
        let conditions = conditions
            .split(",")
            .map(Condition::from_str)
            .collect::<Vec<_>>();

        rules.push(Rule {
            name: name.to_string(),
            conditions,
        });
    }

    for line in itr {
        let values = line[1..line.len() - 1]
            .split(',')
            .map(|str| str[2..].parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        parts.push(Part {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        });
    }

    let mut part1 = 0;
    for part in parts {
        let mut resp = Response::Forward(String::from("in"));

        // Keep matching rules until we get an accept or reject.
        while !matches!(resp, Response::Accept | Response::Reject) {
            if let Response::Forward(ref next) = resp {
                let rule = rules.iter().find(|&r| &r.name == next).unwrap();
                resp = rule.apply(&part);
            }
        }

        if matches!(resp, Response::Accept) {
            part1 += part.x + part.m + part.a + part.s;
        }
    }

    println!("Part one {}", part1);

    let mut valid = Vec::new();
    let mut queue = vec![([(1, 4001); 4], "in".to_owned())];

    while queue.len() > 0 {
        let (mut ranges, rule_name) = queue.remove(0);
        let rule = rules.iter().find(|&r| r.name == rule_name).unwrap();

        for cond in &rule.conditions {
            match &cond.resp {
                Response::NotApplied => panic!("Invalid condition"),
                Response::Compare(ch, op, val, forward) => {
                    let val = *val;
                    let idx = match ch {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!("Invalid char"),
                    };

                    let mut new_range = ranges.clone();
                    let mut rng = ranges.get_mut(idx).unwrap();
                    match op {
                        '<' if rng.0 > val => new_range[idx].0 = new_range[idx].1, // unreachable condition
                        '>' if rng.1 < val => new_range[idx].0 = new_range[idx].1, // unreachable condition
                        '<' if rng.1 > val => {
                            new_range[idx].1 = val;
                            rng.0 = val;
                        },
                        '>' if rng.0 < val => {
                            new_range[idx].0 = val + 1;
                            rng.1 = val + 1;
                        }
                        _ => (),
                    }

                    //println!("Considering {} {} {:?} {:?}", rule.name, forward, new_range, rng);

                    match forward.as_str() {
                        "A" => valid.push(new_range),
                        "R" => (),
                        _ => {
                            queue.push((new_range, forward.to_owned()));
                        }
                    }
                },
                Response::Accept => {
                    //println!("Accept {} {:?}", rule.name, ranges);
                    valid.push(ranges);
                },
                Response::Reject => (),
                Response::Forward(forward) => {
                    queue.push((ranges, forward.to_owned()));
                },
            }
        }
    }

    let part2 = valid
        .iter()
        .map(|arr| arr.iter().map(|(l, h)| i64::from(h - l)).product::<i64>())
        .sum::<i64>();
    println!("Part two {}", part2);

    Ok(())
}