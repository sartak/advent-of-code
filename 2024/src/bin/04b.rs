use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/04-example.txt"
    } else {
        "input/04.txt"
    })?;

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

            let Some(c) = get(x, y) else { continue };
            if c != 'A' {
                continue;
            };

            let Some(tl) = get(x - 1, y - 1) else {
                continue;
            };
            let Some(tr) = get(x + 1, y - 1) else {
                continue;
            };
            let Some(bl) = get(x - 1, y + 1) else {
                continue;
            };
            let Some(br) = get(x + 1, y + 1) else {
                continue;
            };

            // invalid:
            // M.S
            // .A.
            // S.M
            if tl == br {
                continue;
            }

            let m: i32 = [tl, tr, bl, br]
                .into_iter()
                .map(|c| if c == 'M' { 1 } else { 0 })
                .sum();
            let s: i32 = [tl, tr, bl, br]
                .into_iter()
                .map(|c| if c == 'S' { 1 } else { 0 })
                .sum();

            if m == 2 && s == 2 {
                found += 1;
            }
        }
    }

    println!("{found}");

    Ok(())
}
