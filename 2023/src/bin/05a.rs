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

    let mut conversions: HashMap<String, HashMap<String, Vec<(usize, usize, usize)>>> =
        HashMap::new();

    for line in lines.iter() {
        if seeds.is_empty() {
            let (_, s) = line.split_once(": ").unwrap();
            seeds = s
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
        } else if let Some(caps) = rx.captures(line) {
            source = caps.get(1).unwrap().as_str().to_string();
            dest = caps.get(2).unwrap().as_str().to_string();
        } else if line.is_empty() {
            // skip
        } else {
            let v = line
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            let ds = v.get(0).unwrap();
            let sr = v.get(1).unwrap();
            let rl = v.get(2).unwrap();

            let n = (*ds, *sr, *rl);
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

    let mut lowest: Option<usize> = None;
    for seed in seeds {
        let key = String::from("seed");
        let mut queue = vec![(key, seed)];
        while let Some((key, cur)) = queue.pop() {
            if key == "location" {
                if let Some(l) = lowest {
                    if cur < l {
                        lowest = Some(cur);
                    }
                } else {
                    lowest = Some(cur);
                }
            } else {
                let next = conversions.get(&key).unwrap();
                let kv = next.iter().next().unwrap();
                let nk = kv.0;
                let mut seen = false;

                for &(ds, sr, rl) in kv.1 {
                    if cur >= sr && cur < sr + rl {
                        let nn = cur - sr + ds;
                        queue.push((nk.clone(), nn));
                        seen = true;
                        break;
                    }
                }

                if !seen {
                    queue.push((nk.clone(), cur));
                }
            }
        }
    }

    let lowest = lowest.unwrap();
    println!("{lowest}");

    Ok(())
}
