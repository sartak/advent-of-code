use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Outcome {
    Accept,
    Reject,
    Goto(String),
}
use Outcome::*;

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
                        let val = val.parse::<i64>().unwrap();
                        (var, "<", val)
                    } else if let Some((var, val)) = rule.split_once('>') {
                        let val = val.parse::<i64>().unwrap();
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
        workflows.insert(String::from(name), rules);
    }

    let mut ans = 0;
    for line in lines {
        let line = line.strip_prefix('{').unwrap();
        let line = line.strip_suffix('}').unwrap();
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;

        for rating in line.split(',') {
            let (name, score) = rating.split_once('=').unwrap();
            let score = score.parse::<i64>().unwrap();
            match name {
                "x" => x = score,
                "m" => m = score,
                "a" => a = score,
                "s" => s = score,
                _ => panic!(),
            };
        }

        let mut name = String::from("in");
        let ok = 'name: loop {
            let Some(workflow) = workflows.get(&name) else {
                panic!()
            };
            for rule in workflow {
                match rule {
                    (None, Accept) => break 'name true,
                    (None, Reject) => break 'name false,
                    (None, Goto(x)) => {
                        name = x.clone();
                        continue 'name;
                    }
                    (Some((var, op, val)), action) => {
                        let v = match *var {
                            "x" => x,
                            "m" => m,
                            "a" => a,
                            "s" => s,
                            _ => panic!(),
                        };
                        let ok = match *op {
                            "<" => v < *val,
                            ">" => v > *val,
                            _ => panic!(),
                        };
                        if ok {
                            match action {
                                Accept => break 'name true,
                                Reject => break 'name false,
                                Goto(x) => {
                                    name = x.clone();
                                    continue 'name;
                                }
                            }
                        }
                    }
                };
            }

            unreachable!();
        };

        if ok {
            ans += x + m + a + s;
        }
    }

    println!("{ans}");
    Ok(())
}
