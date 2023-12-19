use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;
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

    let mut accepts = vec![];
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
                    accepts.push(cut.clone());
                    break;
                }
                (Some((var, op, val)), Accept) => {
                    let new = new_cut(&cut, var, op, *val);
                    accepts.push(new);

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

    let mut xx = Vec::new();
    let mut mm = Vec::new();
    let mut aa = Vec::new();
    let mut ss = Vec::new();

    for cut in accepts.iter() {
        let Cut {
            xmin,
            xmax,
            mmin,
            mmax,
            amin,
            amax,
            smin,
            smax,
        } = *cut;
        assert!(xmin <= xmax);
        assert!(mmin <= mmax);
        assert!(amin <= amax);
        assert!(smin <= smax);

        xx.push(xmin);
        xx.push(xmax + 1);

        mm.push(mmin);
        mm.push(mmax + 1);

        aa.push(amin);
        aa.push(amax + 1);

        ss.push(smin);
        ss.push(smax + 1);
    }

    xx = xx.into_iter().sorted().unique().collect_vec();
    mm = mm.into_iter().sorted().unique().collect_vec();
    aa = aa.into_iter().sorted().unique().collect_vec();
    ss = ss.into_iter().sorted().unique().collect_vec();

    let ans: u64 = xx
        .par_iter()
        .enumerate()
        .map(|(i, &x0)| {
            let Some(&x1) = xx.get(i + 1) else { return 0 };
            let mut sum = 0;
            for (&m0, &m1) in mm.iter().tuple_windows() {
                for (&a0, &a1) in aa.iter().tuple_windows() {
                    's: for (&s0, &s1) in ss.iter().tuple_windows() {
                        for cut in &accepts {
                            let Cut {
                                xmin,
                                xmax,
                                mmin,
                                mmax,
                                amin,
                                amax,
                                smin,
                                smax,
                            } = *cut;
                            if x0 >= xmin
                                && x1 <= xmax + 1
                                && m0 >= mmin
                                && m1 <= mmax + 1
                                && a0 >= amin
                                && a1 <= amax + 1
                                && s0 >= smin
                                && s1 <= smax + 1
                            {
                                let x = x1 - x0;
                                let m = m1 - m0;
                                let a = a1 - a0;
                                let s = s1 - s0;
                                let combo = x * m * a * s;
                                sum += combo;
                                continue 's;
                            }
                        }
                    }
                }
            }
            sum
        })
        .sum();

    println!("{ans}");
    Ok(())
}
