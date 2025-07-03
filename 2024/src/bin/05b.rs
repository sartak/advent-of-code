use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/05-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/05.txt")?;

    let mut order: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut is_ordering = true;
    let mut answer = 0;

    for line in input.lines() {
        if line.is_empty() {
            is_ordering = false;
        } else if is_ordering {
            let (before, after) = line
                .split('|')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            order.entry(before).or_default().insert(after);
        } else {
            let mut fixed = false;
            let mut pages = line
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            let max = pages.len();

            'check: loop {
                for p in 0..max {
                    let page = pages[p];
                    for a in p + 1..max {
                        let after = pages[a];
                        let Some(must) = order.get(&after) else {
                            continue;
                        };
                        if must.contains(&page) {
                            pages.swap(a, p);
                            fixed = true;
                            continue 'check;
                        }
                    }
                }

                break;
            }

            if fixed {
                answer += pages[pages.len() >> 1];
            }
        }
    }

    println!("{answer}");

    Ok(())
}
