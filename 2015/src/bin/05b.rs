use anyhow::Result;
use fancy_regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/05.txt")?;
    let overlap = Regex::new(r"([a-z][a-z]).*\1")?;
    let repeat = Regex::new(r"([a-z])[a-z]\1")?;
    let res = input
        .lines()
        .filter(|line| overlap.is_match(line).unwrap() && repeat.is_match(line).unwrap())
        .count();
    println!("{res}");
    Ok(())
}
