use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/01.txt")?;
    let words = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let words_pat = words
        .iter()
        .map(|(w, _)| w)
        .copied()
        .collect::<Vec<_>>()
        .join("|");
    let words = words.into_iter().collect::<HashMap<_, _>>();
    let first_rx = Regex::new(&format!(r"(\d|{words_pat})"))?;
    let last_rx = Regex::new(&format!(r".*(\d|{words_pat})"))?;

    let sum: i32 = input
        .lines()
        .map(|line| {
            [&first_rx, &last_rx]
                .into_iter()
                .map(|rx| {
                    let capture = rx.captures(line).unwrap().get(1).unwrap().as_str();
                    if let Some(number) = words.get(capture) {
                        number
                    } else {
                        capture
                    }
                })
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .sum();

    println!("{sum}");
    Ok(())
}
