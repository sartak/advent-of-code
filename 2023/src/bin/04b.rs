use anyhow::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/04.txt")?;
    let input = input.lines().collect::<Vec<_>>();

    let mut cards = vec![1; input.len()];

    input
        .into_iter()
        .map(|card| {
            let (_, nums) = card.split_once(':').unwrap();
            let (winners, have) = nums.split_once('|').unwrap();
            have.split_whitespace()
                .collect::<HashSet<_>>()
                .intersection(&winners.split_whitespace().collect::<HashSet<_>>())
                .count()
        })
        .enumerate()
        .for_each(|(i, sprawl)| {
            let copies = cards[i];
            for j in 1..=sprawl {
                if let Some(card) = cards.get_mut(i + j) {
                    *card += copies;
                }
            }
        });

    let count: usize = cards.into_iter().sum();
    println!("{count}");

    Ok(())
}
