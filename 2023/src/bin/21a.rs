use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/21.txt")?;
    let grid = input
        .lines()
        .map(|row| row.chars().collect_vec())
        .collect_vec();
    let mut sx = 0;
    let mut sy = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                sx = x;
                sy = y;
                break;
            }
        }
    }

    let yy = grid.len();
    let xx = grid[0].len();

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy, 0));

    let target = 64;
    let mut seen = HashSet::new();
    let mut memo = HashSet::new();
    while let Some((x, y, d)) = queue.pop_front() {
        if d == target {
            seen.insert((x, y));
            continue;
        }

        let k = (x, y, d);
        if memo.contains(&k) {
            continue;
        }
        memo.insert(k);

        for dy in -1i64..=1 {
            if y as i64 + dy < 0 {
                continue;
            }
            let y = ((y as i64) + dy) as usize;
            for dx in -1i64..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if dx != 0 && dy != 0 {
                    continue;
                }

                if x as i64 + dx < 0 {
                    continue;
                }
                let x = ((x as i64) + dx) as usize;

                if let Some(row) = grid.get(y) {
                    if let Some(&cell) = row.get(x) {
                        if cell == '.' || cell == 'S' {
                            queue.push_back((x, y, d + 1));
                        }
                    }
                }
            }
        }
    }

    let ans = seen.len();
    println!("{ans}");
    Ok(())
}
