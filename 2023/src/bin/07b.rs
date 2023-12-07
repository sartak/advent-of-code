use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/07.txt")?;

    let ranks = "AKQT98765432J"
        .chars()
        .enumerate()
        .map(|(i, r)| (r, i))
        .collect::<HashMap<_, _>>();

    let winnings: usize = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();
            let hand = hand.chars().collect_vec();

            let order = hand
                .iter()
                .map(|card| ranks.get(card).unwrap())
                .collect_vec();

            let mut counts = hand.iter().counts();
            let jokers = counts.remove(&'J').unwrap_or(0);
            let mut counts = counts.into_values().sorted().rev();
            let top = counts.next().unwrap_or(0);
            let second = counts.next().unwrap_or(0);

            let score = match (top + jokers, second) {
                (5, _) => 0,
                (4, _) => 1,
                (3, 2) => 2,
                (3, _) => 3,
                (2, 2) => 4,
                (2, _) => 5,
                (_, _) => 6,
            };

            (score, order, bid)
        })
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid)
        .sum();

    println!("{winnings}");
    Ok(())
}
