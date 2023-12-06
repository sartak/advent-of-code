use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/06.txt")?;
    let mut lines = input.lines();

    let (times, dist) = [lines.next(), lines.next()]
        .into_iter()
        .map(|line| {
            line.unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
        })
        .next_tuple()
        .unwrap();

    let s: usize = times
        .zip(dist)
        .map(|(t, d)| {
            (0..=t)
                .map(|hold| (t - hold) * hold)
                .filter(|&n| n > d)
                .count()
        })
        .product();

    println!("{s}");
    Ok(())
}
