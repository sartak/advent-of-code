use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/08.txt")?;
    let res: i64 = input
        .lines()
        .map(|line| {
            let encoded: i64 = line
                .chars()
                .batching(|it| match it.next() {
                    None => None,
                    Some('"') => Some(2),
                    Some('\\') => Some(2),
                    Some(_) => Some(1),
                })
                .sum();
            let encoded = encoded + 2; // wrapping quotes
            let len = line.len() as i64;
            encoded - len
        })
        .sum();

    println!("{res}");

    Ok(())
}
