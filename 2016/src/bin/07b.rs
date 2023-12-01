use anyhow::Result;
use fancy_regex::{Captures, Regex};
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/07.txt")?;
    let inner = Regex::new(r"\[(.*?)\]")?;

    let count = input
        .lines()
        .filter(|&line| {
            let mut abas = HashSet::new();
            let outer = inner.replace_all(line, |caps: &Captures| {
                caps.get(1)
                    .unwrap()
                    .as_str()
                    .chars()
                    .tuple_windows()
                    .filter(|&(a, b, c)| a == c && a != b)
                    .for_each(|(a, b, _)| {
                        abas.insert((a, b));
                    });

                "_"
            });

            if abas.is_empty() {
                return false;
            }

            let re = abas
                .into_iter()
                .map(|(a, b)| format!("{b}{a}{b}"))
                .join("|");

            let re = Regex::new(&re).unwrap();
            re.is_match(&outer).unwrap()
        })
        .count();

    println!("{count}");
    Ok(())
}
