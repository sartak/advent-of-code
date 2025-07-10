use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn possible<'a>(cache: &mut HashMap<&'a str, bool>, towels: &[&str], desired: &'a str) -> bool {
    if let Some(&possible) = cache.get(desired) {
        return possible;
    }

    if desired.is_empty() {
        return true;
    }

    let mut p = false;
    for &towel in towels {
        if let Some(rest) = desired.strip_prefix(towel) {
            if possible(cache, towels, rest) {
                p = true;
                break;
            }
        }
    }

    cache.insert(desired, p);

    p
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/19-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/19.txt")?;

    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").collect_vec();
    assert!(lines.next().unwrap().is_empty());

    let mut answer = 0;
    for desired in lines {
        let mut cache = HashMap::new();
        if possible(&mut cache, &towels, desired) {
            answer += 1;
        }
    }

    println!("{answer}");

    Ok(())
}
