use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/03.txt")?;

    let count: usize = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            let chunks = chunks.collect::<Vec<_>>();
            let chunks = (0..chunks.len()).map(|y| {
                (0..chunks[0].len())
                    .map(|x| chunks[x][y])
                    .collect::<Vec<_>>()
            });
            chunks
                .into_iter()
                .filter(|chunk| {
                    let [&a, &b, &c] = &chunk.iter().sorted().collect::<Vec<_>>()[..] else {
                        unreachable!()
                    };
                    a + b > c
                })
                .count()
        })
        .sum();

    println!("{count}");
    Ok(())
}
