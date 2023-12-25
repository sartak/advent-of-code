use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/25.txt")?;
    let lines = input.lines().collect_vec();

    let mut components = HashMap::new();
    let mut cs = Vec::new();
    let mut wires = Vec::new();
    let mut connections = HashMap::new();

    for line in lines.iter() {
        let (left, rights) = line.split_once(": ").unwrap();

        let l = if let Some(l) = components.get(left) {
            *l
        } else {
            let l = components.len();
            cs.push(left);
            components.insert(left, l);
            l
        };

        for right in rights.split_whitespace() {
            let r = if let Some(r) = components.get(right) {
                *r
            } else {
                let r = components.len();
                cs.push(right);
                components.insert(right, r);
                r
            };

            wires.push((l, r));
        }
    }

    for &(l, r) in wires.iter() {
        connections
            .entry(l)
            .and_modify(|v: &mut Vec<usize>| {
                v.push(r);
            })
            .or_insert(vec![r]);
        connections
            .entry(r)
            .and_modify(|v: &mut Vec<usize>| {
                v.push(l);
            })
            .or_insert(vec![l]);
    }

    // magic numbers courtesy of `dot -Kneato -Tsvg`
    let (x1, x2) = (434, 963);
    let (y1, y2) = (254, 256);
    let (z1, z2) = (834, 88);

    let ans = wires
        .par_iter()
        .enumerate()
        .find_map_any(|(i, &(a1, a2))| {
            if !((a1 == x1 && a2 == x2) || (a1 == x2 && a2 == x1)) {
                return None;
            }

            let mut queue = VecDeque::new();

            for (j, &(b1, b2)) in wires.iter().enumerate() {
                if !((b1 == y1 && b2 == y2) || (b1 == y2 && b2 == y1)) {
                    continue;
                }
                if i == j {
                    continue;
                }
                for (k, &(c1, c2)) in wires.iter().enumerate() {
                    if !((c1 == z1 && c2 == z2) || (c1 == z2 && c2 == z1)) {
                        continue;
                    }
                    if i == k || j == k {
                        continue;
                    }

                    queue.clear();
                    let mut red = HashSet::new();
                    for (c, _) in wires.iter().enumerate() {
                        if c == i || c == j || c == k {
                            continue;
                        }
                        queue.push_back(c);
                        break;
                    }
                    assert_eq!(queue.len(), 1);

                    while let Some(from) = queue.pop_front() {
                        red.insert(from);
                        let Some(conns) = connections.get(&from) else {
                            panic!();
                        };
                        for &to in conns {
                            if a1 == from && a2 == to {
                                continue;
                            }
                            if b1 == from && b2 == to {
                                continue;
                            }
                            if c1 == from && c2 == to {
                                continue;
                            }
                            if a1 == to && a2 == from {
                                continue;
                            }
                            if b1 == to && b2 == from {
                                continue;
                            }
                            if c1 == to && c2 == from {
                                continue;
                            }

                            if !red.contains(&to) {
                                queue.push_back(to);
                            }
                        }
                    }

                    // only one subgraph
                    if red.len() == components.len() {
                        continue;
                    }

                    let mut blue = HashSet::new();
                    for (c, _) in wires.iter().enumerate() {
                        if c == i || c == j || c == k {
                            continue;
                        }
                        if red.contains(&c) {
                            continue;
                        }
                        queue.push_back(c);
                        break;
                    }
                    assert_eq!(queue.len(), 1);

                    while let Some(from) = queue.pop_front() {
                        blue.insert(from);
                        let Some(conns) = connections.get(&from) else {
                            panic!();
                        };
                        for &to in conns {
                            if a1 == from && a2 == to {
                                continue;
                            }
                            if b1 == from && b2 == to {
                                continue;
                            }
                            if c1 == from && c2 == to {
                                continue;
                            }
                            if a1 == to && a2 == from {
                                continue;
                            }
                            if b1 == to && b2 == from {
                                continue;
                            }
                            if c1 == to && c2 == from {
                                continue;
                            }

                            if !blue.contains(&to) {
                                queue.push_back(to);
                            }
                        }
                    }

                    if red.len() + blue.len() == components.len() {
                        return Some(red.len() * blue.len());
                    }
                }
            }
            None
        })
        .unwrap();

    println!("{ans}");

    Ok(())
}
