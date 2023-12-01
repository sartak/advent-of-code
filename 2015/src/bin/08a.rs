use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/08.txt")?;
    let res: i64 = input
        .lines()
        .map(|line| {
            let mem: i64 = line
                .chars()
                .batching(|it| match it.next() {
                    None => None,
                    Some('\\') => match it.next() {
                        None => Some(1),
                        Some('\\') => Some(1),
                        Some('"') => Some(1),
                        Some('x') => {
                            it.next();
                            it.next();
                            Some(1)
                        }
                        Some(_) => Some(2),
                    },
                    Some(_) => Some(1),
                })
                .sum();
            let mem = mem - 2; // wrapping quotes
            let len = line.len() as i64;
            len - mem
        })
        .sum();

    println!("{res}");

    Ok(())
}
