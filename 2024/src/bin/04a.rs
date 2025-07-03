use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/04-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/04.txt")?;

    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let get = |x: i64, y: i64| -> Option<char> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        let line: &Vec<char> = grid.get(y)?;
        Some(*line.get(x)?)
    };

    let mut found = 0;

    for y in 0..grid.len() {
        let y = y as i64;
        for x in 0..grid[y as usize].len() {
            let x = x as i64;
            for (dx, dy) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let Some(chars) = (0..=3)
                    .map(|n| get(x + dx * n, y + dy * n))
                    .collect::<Option<String>>()
                else {
                    continue;
                };
                if chars == "XMAS" {
                    found += 1;
                }
            }
        }
    }

    println!("{found}");

    Ok(())
}
