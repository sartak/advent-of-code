use anyhow::Result;
use fancy_regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/07.txt")?;
    let inner = Regex::new(r"\[(.*?)\]")?;
    let abba = Regex::new(r"([a-z])((?!\1)[a-z])\2\1")?;

    let count = input
        .lines()
        .filter(|line| {
            if inner.captures_iter(line).any(|caps| {
                let inner = caps.unwrap().get(1).unwrap().as_str();
                abba.is_match(inner).unwrap()
            }) {
                false
            } else {
                abba.is_match(line).unwrap()
            }
        })
        .count();

    println!("{count}");
    Ok(())
}
