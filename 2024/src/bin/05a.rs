use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/05-example.txt"
    } else {
        "input/05.txt"
    })?;

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
            let mut safe = true;
            let pages = line
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            for (p, page) in pages.iter().enumerate() {
                for after in &pages[p + 1..] {
                    let Some(must) = order.get(after) else {
                        continue;
                    };
                    if must.contains(page) {
                        safe = false;
                    }
                }
            }

            if safe {
                answer += pages[pages.len() >> 1];
            }
        }
    }

    println!("{answer}");

    Ok(())
}
