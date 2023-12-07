use anyhow::Result;
use fancy_regex::Regex;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/07.txt")?;
    let lines = input.lines().collect_vec();
    let five = Regex::new(r"(.)\1\1\1\1")?;
    let four = Regex::new(r"(.)\1\1\1")?;
    let fh = Regex::new(r"(.)\1\1(.)\2")?;
    let fh2 = Regex::new(r"(.)\1(.)\2\2")?;
    let three = Regex::new(r"(.)\1\1")?;
    let twopair = Regex::new(r"(.)\1.?(.)\2")?;
    let onepair = Regex::new(r"(.)\1")?;
    let mut s: i64 = 0;

    let ranks = vec![
        "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
    ];

    let mut hands = lines
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let mut h = hand.chars().map(String::from).collect_vec();
            let hand = h.clone();
            h.sort();
            let h = h.join("");
            let bid = bid.parse::<i64>().unwrap();

            let score = if five.is_match(&h).unwrap() {
                0
            } else if four.is_match(&h).unwrap() {
                1
            } else if fh.is_match(&h).unwrap() || fh2.is_match(&h).unwrap() {
                2
            } else if three.is_match(&h).unwrap() {
                3
            } else if twopair.is_match(&h).unwrap() {
                4
            } else if onepair.is_match(&h).unwrap() {
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
