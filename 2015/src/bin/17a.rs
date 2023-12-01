use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/17.txt")?;
    let amount = 150;
    let containers = input
        .lines()
        .map(|l| Ok(l.parse::<usize>()?))
        .collect::<Result<Vec<_>>>()?;

    let mut count = 0;
    for n in 1..=containers.len() {
        for combo in containers.iter().combinations(n) {
            let size: usize = combo.into_iter().sum();
            if size == amount {
                count += 1
            }
        }
    }

    println!("{count}");

    Ok(())
}
