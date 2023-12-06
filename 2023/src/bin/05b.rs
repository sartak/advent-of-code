use anyhow::Result;
use itertools::Itertools;
use maplit::hashmap as map;
use rangemap::RangeMap;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/05.txt")?;
    let lines = input.lines().collect_vec();
    let rx = Regex::new(r"^(\w+)-to-(\w+) map:")?;

    let mut seeds = Vec::new();
    let mut source = String::from("");
    let mut dest = String::from("");

    let mut conversions: HashMap<String, HashMap<String, RangeMap<_, _>>> = HashMap::new();

    for line in lines.iter() {
        if seeds.is_empty() {
            let (_, s) = line.split_once(": ").unwrap();
            seeds = s
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .tuples::<(_, _)>()
                .map(|(start, len)| start..(start + len))
                .collect_vec();
        } else if let Some(caps) = rx.captures(line) {
            source = caps.get(1).unwrap().as_str().to_string();
            dest = caps.get(2).unwrap().as_str().to_string();
        } else if line.is_empty() {
            // skip
        } else {
            let v = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec();
            let ds = *v.get(0).unwrap();
            let sr = *v.get(1).unwrap();
            let rl = *v.get(2).unwrap();
            let range = sr..(sr + rl);
            let offset = ds - sr;

            conversions
                .entry(source.clone())
                .and_modify(|d| {
                    d.entry(dest.clone())
                        .and_modify(|r| {
                            r.insert(range.clone(), offset);
                        })
                        .or_insert([(range.clone(), offset)].into_iter().collect());
                })
                .or_insert({
                    map! { dest.clone() => [(range, offset)].into_iter().collect() }
                });
        }
    }

    let mut lowest: Option<i64> = None;
    for seed in seeds {
        let mut queue = vec![(String::from("seed"), seed)];
        while let Some((key, range)) = queue.pop() {
            if key == "location" {
                if let Some(l) = lowest {
                    if range.start < l {
                        lowest = Some(range.start);
                    }
                } else {
                    lowest = Some(range.start);
                }
            } else {
                let next = conversions.get(&key).unwrap();
                let (nk, ranges) = next.iter().next().unwrap();
                for gap in ranges.gaps(&range) {
                    queue.push((nk.clone(), gap));
                }
                for (overlap, offset) in ranges.overlapping(&range) {
                    let start = overlap.start.max(range.start) + offset;
                    let end = overlap.end.min(range.end) + offset;
                    queue.push((nk.clone(), start..end));
                }
            }
        }
    }

    let lowest = lowest.unwrap();
    println!("{lowest}");

    Ok(())
}
