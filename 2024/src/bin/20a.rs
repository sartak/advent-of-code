use anyhow::Result;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[derive(Clone, Copy)]
enum Cell {
    Wall,
    Empty,
    Victory,
}
use Cell::*;

fn path(
    grid: &[Vec<Cell>],
    sx: usize,
    sy: usize,
    cheat: Option<usize>,
    baseline: Option<usize>,
) -> Vec<usize> {
    let mut scores = Vec::new();
    let height = grid.len();
    let width = grid[0].len();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), sx, sy, cheat, 0, None));

    let mut seen = HashSet::new();

    while let Some((score, x, y, cheat, phased, location)) = queue.pop() {
        let score = score.0;

        if let Some(b) = baseline {
            if score > b {
                continue;
            }
        }

        if !seen.insert((x, y, location)) {
            continue;
        }

        let cell = grid[y][x];
        let mut activate = None;
        match cell {
            Empty => {}
            Wall => {
                if phased > 1 {
                    // fully phased
                } else if phased == 1 {
                    // ending phase but we're in a wall so we crash
                    continue;
                } else if cheat.is_some() {
                    // not phased, but we start phasing now
                    activate = cheat;
                } else {
                    // not phased and we can't, so skip
                    continue;
                }
            }
            Victory => {
                scores.push(score);
                continue;
            }
        }

        let ox = x;
        let oy = y;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = x as i64 + dx;
            let y = y as i64 + dy;

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= width || y >= height {
                continue;
            }

            if let Some(duration) = activate {
                queue.push((Reverse(score + 1), x, y, None, duration - 1, Some((ox, oy))));
            } else {
                queue.push((
                    Reverse(score + 1),
                    x,
                    y,
                    cheat,
                    phased.saturating_sub(1),
                    location,
                ));
            }
        }
    }

    scores
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/20-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/20.txt")?;

    #[cfg(debug_assertions)]
    let threshold = 2;
    #[cfg(not(debug_assertions))]
    let threshold = 100;

    let mut sx = 0;
    let mut sy = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            let cell = match c {
                '.' => Empty,
                '#' => Wall,
                'E' => Victory,
                'S' => {
                    sx = row.len();
                    sy = grid.len();
                    Empty
                }
                _ => panic!("Unhandled character {c}"),
            };
            row.push(cell);
        }
        grid.push(row);
    }

    let baseline = path(&grid, sx, sy, None, None);
    assert_eq!(baseline.len(), 1);
    let baseline = baseline[0];

    let answer = path(&grid, sx, sy, Some(2), Some(baseline - threshold)).len();

    println!("{answer}");

    Ok(())
}
