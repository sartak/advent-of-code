use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/24.txt")?;
    let partitions = 4;
    let weights = input
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<Result<Vec<usize>>>()?;
    let target = weights.iter().sum::<usize>() / partitions;

    let mut best = None;
    for size in 1..weights.len() {
        for combo in weights.iter().combinations(size) {
            let sum: usize = combo.iter().copied().sum();
            if sum != target {
                continue;
            }

            let qe: usize = combo.iter().copied().product();
            if let Some(best) = best {
                if qe > best {
                    continue;
                }
            }

            best = Some(qe);
        }

        if best.is_some() {
            break;
        }
    }

    let best = best.unwrap();
    eprintln!("{best}");

    Ok(())
}
