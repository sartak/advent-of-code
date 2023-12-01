use anyhow::Result;
use fancy_regex::Regex as Fregex;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/05.txt")?;
    let vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]")?;
    let double = Fregex::new(r"([a-z])\1")?;
    let bad = Regex::new(r"ab|cd|pq|xy")?;
    let res = input
        .lines()
        .filter(|line| {
            vowels.is_match(line) && double.is_match(line).unwrap() && !bad.is_match(line)
        })
        .count();
    println!("{res}");
    Ok(())
}
