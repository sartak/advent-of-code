use anyhow::Result;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[derive(Clone, Copy)]
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
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/18-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/18.txt")?;

    #[cfg(debug_assertions)]
    let size = 6 + 1;
    #[cfg(not(debug_assertions))]
    let size = 70 + 1;

    let lines = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|i| i.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let empty = (0..size)
        .map(|_| (0..size).map(|_| Cell::Empty).collect_vec())
        .collect_vec();

    let mut min = 0;
    let mut max = lines.len();

    while min < max {
        let step = (min + max) / 2;

        let mut grid = empty.clone();
        for (x, y) in &lines[..step] {
            grid[*y][*x] = Wall;
        }
        let score = path(&grid, 0, 0, size - 1, size - 1);
        if score.is_some() {
            min = step + 1;
        } else {
            max = step;
        }
    }

    let (x, y) = lines[max - 1];
    println!("{x},{y}");
    Ok(())
}
