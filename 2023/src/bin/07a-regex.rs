use std::collections::HashMap;

use anyhow::Result;
use fancy_regex::Regex;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/07.txt")?;

    let five = Regex::new(r"(.)\1\1\1\1")?;
    let four = Regex::new(r"(.)\1\1\1")?;
    let fullhouse = Regex::new(r"(.)\1\1(.)\2|(.)\3(.)\4\4")?;
    let three = Regex::new(r"(.)\1\1")?;
    let twopair = Regex::new(r"(.)\1.?(.)\2")?;
    let onepair = Regex::new(r"(.)\1")?;
    let kinds = [&five, &four, &fullhouse, &three, &twopair, &onepair];

    let ranks = "AKQJT98765432"
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

            let hand = hand.into_iter().sorted().join("");

            let score = kinds
                .into_iter()
                .enumerate()
                .find_map(|(i, rx)| {
                    if rx.is_match(&hand).unwrap() {
                        Some(i)
                    } else {
                        None
                    }
                })
                .unwrap_or(6);

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
