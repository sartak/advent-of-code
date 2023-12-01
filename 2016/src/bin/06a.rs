use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/06.txt")?
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let message = (0..input[0].len())
        .map(|i| {
            let counts = input.iter().map(|chars| chars.get(i).unwrap()).counts();
            counts.into_iter().max_by_key(|&(_, n)| n).unwrap().0
        })
        .collect::<String>();

    println!("{message}");
    Ok(())
}
