use anyhow::Result;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

enum Cell {
    Wall,
    Empty,
}
use Cell::*;
use itertools::Itertools;

fn path(grid: &[Vec<Cell>], sx: usize, sy: usize, ex: usize, ey: usize) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), sx, sy));

    let mut seen = HashSet::new();

    while let Some((score, x, y)) = queue.pop() {
        if seen.contains(&(x, y)) {
            continue;
        }
        seen.insert((x, y));

        let score = score.0;
        if x == ex && y == ey {
            return Some(score);
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = x as i64 + dx;
            let y = y as i64 + dy;

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            let Some(line) = grid.get(y) else {
                continue;
            };

            let Some(cell) = line.get(x) else {
                continue;
            };

            match cell {
                Wall => continue,
                Empty => {}
            }

            queue.push((Reverse(score + 1), x, y));
        }
    }

    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/18-example.txt"
    } else {
        "input/18.txt"
    })?;

    let size = if cfg!(debug_assertions) {
        6 + 1
    } else {
        70 + 1
    };

    let corrupts = if cfg!(debug_assertions) { 12 } else { 1024 };

    let mut grid = (0..size)
        .map(|_| (0..size).map(|_| Cell::Empty).collect_vec())
        .collect_vec();

    let mut lines = input.lines();
    for _ in 0..corrupts {
        let line = lines.next().unwrap();
        let (x, y) = line
            .split(',')
            .map(|i| i.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        grid[y][x] = Wall;
    }

    let score = path(&grid, 0, 0, size - 1, size - 1).unwrap();
    println!("{score}");

    Ok(())
}
