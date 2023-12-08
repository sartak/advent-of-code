use anyhow::Result;
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/08.txt")?;
    let mut lines = input.lines();
    let rx = Regex::new(r"(\w+) = \((\w+), (\w+)\)")?;

    let dirs = lines.next().unwrap();

    let _empty = lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let caps = rx.captures(line).unwrap();
        let start = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();

        map.insert(start, (left, right));
    }

    let starts = map
        .keys()
        .filter(|pos| pos.ends_with('A'))
        .copied()
        .collect_vec();

    let mut mins = HashMap::new();

    for &start in starts.iter() {
        let mut pos = start;
        for (i, dir) in dirs.chars().cycle().enumerate() {
            let options = map.get(pos).unwrap();
            let next = match dir {
                'L' => options.0,
                'R' => options.1,
                _ => panic!(),
            };
            let key = (start, next);
            if next.ends_with('Z') && !mins.contains_key(&key) {
                mins.insert(key, i + 1);

                // in the comp i used a very large number to break, but since
                // each origin reaches only one destination, we can bail out
                break;
            }

            pos = next;
        }
    }

    let cycles = starts
        .iter()
        .map(|pos| {
            mins.iter()
                .filter_map(
                    |((start, _), steps)| {
                        if start == pos {
                            Some(steps)
                        } else {
                            None
                        }
                    },
                )
                .collect_vec()
        })
        .collect_vec();

    // i noticed each cycle was one length long
    let res = cycles
        .into_iter()
        .map(|cycle| **cycle.first().unwrap())
        .fold(1, lcm);
    println!("{res}");

    Ok(())
}
