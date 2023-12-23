use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/23.txt")?;
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let sy = 0usize;
    let sx = grid[0].iter().position(|&c| c == '.').unwrap();

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy, vec![(sx, sy)]));

    let mut max = 0;
    while let Some((x, y, path)) = queue.pop_front() {
        max = max.max(path.len() - 1);

        for dy in -1..=1 {
            if y as i64 + dy < 0 {
                continue;
            }
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                if dx != 0 && dy != 0 {
                    continue;
                }

                if x as i64 + dx < 0 {
                    continue;
                }

                let y = (y as i64 + dy) as usize;
                let x = (x as i64 + dx) as usize;
                if let Some(row) = grid.get(y) {
                    if let Some(cell) = row.get(x) {
                        let ok = match cell {
                            '#' => continue,
                            '.' => true,
                            '^' => dy == -1,
                            '<' => dx == -1,
                            '>' => dx == 1,
                            'v' => dy == 1,
                            _ => panic!(),
                        };
                        if !ok {
                            continue;
                        }

                        if path.contains(&(x, y)) {
                            continue;
                        }

                        let mut path = path.clone();
                        path.push((x, y));
                        queue.push_back((x, y, path));
                    }
                }
            }
        }
    }

    println!("{max}");

    Ok(())
}
