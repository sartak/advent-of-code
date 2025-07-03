use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/01-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/01.txt")?;

    let mut left = Vec::new();
    let mut right = HashMap::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let l = words.next().unwrap();
        let r = words.next().unwrap();
        assert!(words.next().is_none());

        left.push(l.parse::<i32>()?);
        right
            .entry(r.parse::<i32>()?)
            .and_modify(|r| *r += 1)
            .or_insert(1);
    }

    let mut dist = 0;
    for l in left {
        let d = l * right.get(&l).unwrap_or(&0);
        dist += d;
    }

    println!("{dist}");

    Ok(())
}
