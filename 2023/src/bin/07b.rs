use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/07.txt")?;
    let lines = input.lines().collect_vec();
    let mut s: i64 = 0;

    let ranks = vec![
        "A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J",
    ];

    let mut hands = lines
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = hand.chars().map(String::from).collect_vec();
            let bid = bid.parse::<i64>().unwrap();

            let mut hm = HashMap::new();
            for c in hand.iter() {
                let key = String::from(c);
                hm.entry(key).and_modify(|v| *v += 1).or_insert(1);
            }
            let jokers = hm.remove("J").unwrap_or(0);
            let hm = hm.into_values().sorted_by_key(|v| -v).collect_vec();
            let a = *hm.get(0).unwrap_or(&0) + jokers;
            let b = *hm.get(1).unwrap_or(&0);

            let score = if a == 5 {
                0
            } else if a == 4 {
                1
            } else if a == 3 && b == 2 {
                2
            } else if a == 3 {
                3
            } else if a == 2 && b == 2 {
                4
            } else if a == 2 {
                5
            } else {
                6
            };

            let order = hand
                .into_iter()
                .map(|c| ranks.iter().position(|n| *n == c).unwrap())
                .collect_vec();

            (score, order, bid)
        })
        .collect_vec();

    hands.sort();
    hands.reverse();
    for (i, (_, _, bid)) in hands.into_iter().enumerate() {
        s += (i as i64 + 1) * bid;
    }

    println!("{s}");
    Ok(())
}
