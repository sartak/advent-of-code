use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/03.txt")?;
    let input = input.lines().collect_vec();
    let rx = Regex::new(r"[0-9]+")?;

    let symbols = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c.is_ascii_digit() || c == '.' {
                    None
                } else {
                    Some((x, y))
                }
            })
        })
        .collect::<HashSet<_>>();

    let sum = input
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            rx.find_iter(line)
                .filter_map(|res| {
                    let num = res.as_str();
                    let x0 = res.start();
                    let x1 = x0 + num.len();
                    let num = num.parse::<usize>().unwrap();

                    if (x0.saturating_sub(1)..=x1)
                        .cartesian_product(y.saturating_sub(1)..=(y + 1))
                        .any(|(x, y)| symbols.contains(&(x, y)))
                    {
                        Some(num)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{sum}");
    Ok(())
}
