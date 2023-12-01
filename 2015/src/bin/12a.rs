use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/12.txt")?;
    let regex = Regex::new(r"-?\d+")?;
    let res: i64 = regex
        .find_iter(&input)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .sum();

    println!("{res}");

    Ok(())
}
