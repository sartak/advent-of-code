use anyhow::Result;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{
        BinaryHeap, {HashMap, HashSet},
    },
};

#[derive(Clone, Copy)]
enum Cell {
    Wall,
    Empty,
}
use Cell::*;

fn cheats(
    grid: &[Vec<Cell>],
    steps: &[Vec<(Option<usize>, Option<usize>)>],
    maximum: usize,
    duration: usize,
) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut wins = 0;

    for (oy, row) in grid.iter().enumerate() {
        for (ox, cell) in row.iter().enumerate() {
            match cell {
                Empty => {}
                Wall => continue,
            }

            let entry_steps = steps[oy][ox].0.unwrap();

            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0), ox, oy));

            let mut exits = HashMap::new();
            let mut seen = HashSet::new();

            while let Some((steps, x, y)) = queue.pop() {
                let steps = steps.0;

                if steps == duration {
                    continue;
                }

                if !seen.insert((x, y)) {
                    continue;
                }

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

                    match grid[y][x] {
                        Empty => {
                            exits.entry((x, y)).or_insert(steps + 1);
                        }
                        Wall => {}
                    }
                    queue.push((Reverse(steps + 1), x, y));
                }
            }

            for ((ex, ey), cheat_steps) in exits {
                let exit_steps = steps[ey][ex].1.unwrap();
                if entry_steps + cheat_steps + exit_steps <= maximum {
                    wins += 1;
                }
            }
        }
    }

    wins
}

fn prepare(
    grid: &[Vec<Cell>],
    sx: usize,
    sy: usize,
    ex: usize,
    ey: usize,
) -> Vec<Vec<(Option<usize>, Option<usize>)>> {
    let mut scored = grid
        .iter()
        .map(|row| row.iter().map(|_| (None, None)).collect_vec())
        .collect_vec();

    for (ox, oy, is_start) in [(sx, sy, true), (ex, ey, false)] {
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), ox, oy));

        let mut seen = HashSet::new();

        if is_start {
            scored[oy][ox].0 = Some(0);
        } else {
            scored[oy][ox].1 = Some(0);
        }
        seen.insert((ox, oy));

        while let Some((steps, x, y)) = queue.pop() {
            let steps = steps.0 + 1;

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = x as i16 + dx;
                let y = y as i16 + dy;
                if x < 0 || y < 0 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;

                if !seen.insert((x, y)) {
                    continue;
                }

                match grid[y][x] {
                    Wall => continue,
                    Empty => {
                        if is_start {
                            scored[y][x].0 = Some(steps);
                        } else {
                            scored[y][x].1 = Some(steps);
                        }
                    }
                }

                queue.push((Reverse(steps), x, y));
            }
        }
    }

    scored
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/20-example.txt"
    } else {
        "input/20.txt"
    })?;

    let threshold = if cfg!(debug_assertions) { 50 } else { 100 };

    let mut sx = 0;
    let mut sy = 0;
    let mut ex = 0;
    let mut ey = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            let cell = match c {
                '.' => Empty,
                '#' => Wall,
                'E' => {
                    ex = row.len();
                    ey = grid.len();
                    Empty
                }
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

    let steps = prepare(&grid, sx, sy, ex, ey);
    let baseline = steps[sy][sx].1.unwrap();

    let answer = cheats(&grid, &steps, baseline - threshold, 20);
    println!("{answer}");

    Ok(())
}
