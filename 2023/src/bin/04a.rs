use anyhow::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/04.txt")?;

    let sum: u64 = input
        .lines()
        .map(|card| {
            let (_, nums) = card.split_once(':').unwrap();
            let (winners, have) = nums.split_once('|').unwrap();
            let count = have
                .split_whitespace()
                .collect::<HashSet<_>>()
                .intersection(&winners.split_whitespace().collect::<HashSet<_>>())
                .count();

            1 << (count - 1)
        })
        .sum();

    println!("{sum}");
    Ok(())
}
