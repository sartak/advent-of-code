use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Outcome {
    Accept,
    Reject,
    Goto(String),
}
use Outcome::*;

#[derive(Debug, Clone)]
struct Cut {
    xmin: u64,
    xmax: u64,
    mmin: u64,
    mmax: u64,
    amin: u64,
    amax: u64,
    smin: u64,
    smax: u64,
}

fn anti_cut(cut: &Cut, var: &str, op: &str, val: u64) -> Cut {
    let (op, val) = if op == "<" {
        (">", val - 1)
    } else {
        ("<", val + 1)
    };
    new_cut(cut, var, op, val)
}

fn new_cut(cut: &Cut, var: &str, op: &str, val: u64) -> Cut {
    match (var, op) {
        ("x", "<") => Cut {
            xmax: cut.xmax.min(val - 1),
            ..*cut
        },
        ("x", ">") => Cut {
            xmin: cut.xmin.max(val + 1),
            ..*cut
        },
        ("m", "<") => Cut {
            mmax: cut.mmax.min(val - 1),
            ..*cut
        },
        ("m", ">") => Cut {
            mmin: cut.mmin.max(val + 1),
            ..*cut
        },
        ("a", "<") => Cut {
            amax: cut.amax.min(val - 1),
            ..*cut
        },
        ("a", ">") => Cut {
            amin: cut.amin.max(val + 1),
            ..*cut
        },
        ("s", "<") => Cut {
            smax: cut.smax.min(val - 1),
            ..*cut
        },
        ("s", ">") => Cut {
            smin: cut.smin.max(val + 1),
            ..*cut
        },
        (_, _) => panic!(),
    }
}

fn score(cut: &Cut) -> u64 {
    let Cut {
        xmin,
        xmax,
        mmin,
        mmax,
        amin,
        amax,
        smin,
        smax,
    } = cut;
    (xmax - xmin + 1) * (mmax - mmin + 1) * (amax - amin + 1) * (smax - smin + 1)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/19.txt")?;
    let mut lines = input.lines();

    let mut workflows = HashMap::new();
    let rx = Regex::new(r"^(\w+)\{(.+)\}$")?;

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let caps = rx.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let rules = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(|r| {
                if let Some((rule, target)) = r.split_once(':') {
                    let rule = if let Some((var, val)) = rule.split_once('<') {
                        let val = val.parse::<u64>().unwrap();
                        (var, "<", val)
                    } else if let Some((var, val)) = rule.split_once('>') {
                        let val = val.parse::<u64>().unwrap();
                        (var, ">", val)
                    } else {
                        panic!("{r}");
                    };
                    let target = match target {
                        "A" => Accept,
                        "R" => Reject,
                        x => Goto(String::from(x)),
                    };
                    (Some(rule), target)
                } else {
                    let target = match r {
                        "A" => Accept,
                        "R" => Reject,
                        x => Goto(String::from(x)),
                    };
                    (None, target)
                }
            })
            .collect_vec();
        workflows.insert(name, rules);
    }

    let mut queue = VecDeque::new();
    queue.push_back((
        "in",
        Cut {
            xmin: 1,
            xmax: 4000,
            mmin: 1,
            mmax: 4000,
            amin: 1,
            amax: 4000,
            smin: 1,
            smax: 4000,
        },
    ));

    let mut ans = 0;
    while let Some((name, mut cut)) = queue.pop_front() {
        let Some(rules) = workflows.get(name) else {
            panic!()
        };

        for rule in rules {
            match rule {
                (None, Reject) => {
                    break;
                }
                (None, Accept) => {
                    ans += score(&cut);
                    break;
                }
                (Some((var, op, val)), Accept) => {
                    let new = new_cut(&cut, var, op, *val);
                    ans += score(&new);

                    cut = anti_cut(&cut, var, op, *val);
                }
                (Some((var, op, val)), Reject) => {
                    cut = anti_cut(&cut, var, op, *val);
                }
                (None, Goto(next)) => {
                    queue.push_back((next, cut.clone()));
                    break;
                }
                (Some((var, op, val)), Goto(next)) => {
                    let new = new_cut(&cut, var, op, *val);
                    queue.push_back((next, new));

                    cut = anti_cut(&cut, var, op, *val);
                }
            }
        }
    }

    println!("{ans}");
    Ok(())
}
