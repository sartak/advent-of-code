use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/03.txt")?;

    let count = input
        .lines()
        .filter(|line| {
            let mut sides = line
                .split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            sides.sort();
            let &[a, b, c] = &sides[..] else {
                unreachable!()
            };
            a + b > c
        })
        .count();

    println!("{count}");
    Ok(())
}
