use anyhow::Result;
use itertools::Itertools;
use maplit::hashmap as map;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/05.txt")?;
    let lines = input.lines().collect_vec();
    let rx = Regex::new(r"^(\w+)-to-(\w+) map:")?;

    let mut seeds = Vec::new();
    let mut source = String::from("");
    let mut dest = String::from("");

    let mut conversions: HashMap<String, HashMap<String, Vec<(i64, i64, i64)>>> = HashMap::new();

    for line in lines.iter() {
        if seeds.is_empty() {
            let (_, s) = line.split_once(": ").unwrap();
            seeds = s
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .tuples::<(_, _)>()
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
            let ds = v.get(0).unwrap();
            let sr = v.get(1).unwrap();
            let rl = v.get(2).unwrap();

            let n = (*sr, *ds, *rl);
            conversions
                .entry(source.clone())
                .and_modify(|d| {
                    d.entry(dest.clone())
                        .and_modify(|v| {
                            v.push(n);
                            v.sort();
                        })
                        .or_insert(vec![n]);
                })
                .or_insert({
                    map! { dest.clone() => vec![n] }
                });
        }
    }

    let mut lowest: Option<i64> = None;
    for (start, len) in seeds {
        let mut queue = vec![(String::from("seed"), start, len)];
        while let Some((key, start, len)) = queue.pop() {
            if key == "location" {
                if let Some(l) = lowest {
                    if start < l {
                        lowest = Some(start);
                    }
                } else {
                    lowest = Some(start);
                }
            } else {
                let next = conversions.get(&key).unwrap();
                let kv = next.iter().next().unwrap();
                let nk = kv.0;
                let vs = kv.1;
                let mut vi = 0;

                while let Some(&(sr, ds, rl)) = vs.get(vi) {
                    let map = ds - sr;
                    if start < sr {
                        if start + len < sr {
                            queue.push((nk.clone(), start, len));
                        } else {
                            let front = sr - start;
                            let back = len - front;
                            queue.push((nk.clone(), start, front));
                            queue.push((key.clone(), sr, back));
                        }
                        break;
                    } else if start == sr {
                        if len <= rl {
                            queue.push((nk.clone(), start + map, len));
                        } else {
                            queue.push((nk.clone(), start + map, rl));
                            queue.push((key.clone(), start + rl, len - rl));
                        }
                        break;
                    } else if start >= sr + rl {
                        vi += 1;
                        if vs.get(vi).is_none() {
                            queue.push((nk.clone(), start, len));
                        }
                    } else {
                        if start + len < sr + rl {
                            queue.push((nk.clone(), start + map, len));
                        } else {
                            let nl = sr + rl - start;
                            queue.push((nk.clone(), start + map, nl));
                            queue.push((key.clone(), start + nl, len - nl));
                        }
                        break;
                    }
                }
            }
        }
    }

    let lowest = lowest.unwrap();
    println!("{lowest}");

    Ok(())
}
