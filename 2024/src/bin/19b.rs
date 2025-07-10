use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn score<'a>(cache: &mut HashMap<&'a str, usize>, towels: &[&str], desired: &'a str) -> usize {
    if let Some(&score) = cache.get(desired) {
        return score;
    }

    if desired.is_empty() {
        return 1;
    }

    let mut s = 0;
    for &towel in towels {
        if let Some(rest) = desired.strip_prefix(towel) {
            s += score(cache, towels, rest);
        }
    }

    cache.insert(desired, s);

    s
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/19-example.txt"
    } else {
        "input/19.txt"
    })?;

    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").collect_vec();
    assert!(lines.next().unwrap().is_empty());

    let mut answer = 0;
    for desired in lines {
        let mut cache = HashMap::new();
        answer += score(&mut cache, &towels, desired);
    }

    println!("{answer}");

    Ok(())
}
