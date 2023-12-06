use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/06.txt")?;
    let mut lines = input.lines();

    let count = [lines.next(), lines.next()]
        .into_iter()
        .map(|line| {
            line.unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .join("")
                .parse::<usize>()
                .unwrap()
        })
        .tuples()
        .map(|(time, best)| {
            (0..=time)
                .map(|hold| (time - hold) * hold)
                .filter(|&dist| dist > best)
                .count()
        })
        .next()
        .unwrap();

    println!("{count}");
    Ok(())
}
