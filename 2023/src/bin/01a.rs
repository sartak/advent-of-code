use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/01.txt")?;
    let first_rx = Regex::new(r"(\d)")?;
    let last_rx = Regex::new(r".*(\d)")?;

    let sum: i32 = input
        .lines()
        .map(|line| {
            [&first_rx, &last_rx]
                .into_iter()
                .map(|rx| rx.captures(line).unwrap().get(1).unwrap().as_str())
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .sum();

    println!("{sum}");
    Ok(())
}
