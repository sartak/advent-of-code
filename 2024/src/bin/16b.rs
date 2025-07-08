use anyhow::Result;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Empty,
    Victory,
}
use Cell::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}
use Direction::*;

impl Direction {
    fn as_delta(&self) -> (i64, i64) {
        match self {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
    }

    fn clock(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn counter(&self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

fn path(grid: &[Vec<Cell>], sx: usize, sy: usize, sd: Direction) -> usize {
    let score_advance = 1;
    let score_turn = 1000;
    let height = grid.len();
    let width = grid[0].len();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), sx, sy, sd, Vec::new()));

    let mut best = HashMap::new();
    let mut paths = Vec::new();
    let mut result = None;

    while let Some((score, x, y, dir, path)) = queue.pop() {
        let score = score.0;

        if let Some(r) = result {
            if score > r {
                continue;
            }
        }

        if let Some(&b) = best.get(&(x, y, dir)) {
            if score > b {
                continue;
            }
        }
        best.insert((x, y, dir), score);

        let mut path = path.clone();
        path.push((x, y));

        let cell = grid[y][x];
        match cell {
            Wall => continue,
            Empty => {}
            Victory => {
                result = Some(score);
                paths.push(path);
                continue;
            }
        }

        queue.push((Reverse(score + score_turn), x, y, dir.clock(), path.clone()));
        queue.push((
            Reverse(score + score_turn),
            x,
            y,
            dir.counter(),
            path.clone(),
        ));

        let (dx, dy) = dir.as_delta();
        let nx = x as i64 + dx;
        let ny = y as i64 + dy;
        if nx < 0 || ny < 0 {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if nx >= width || ny >= height {
            continue;
        }

        queue.push((Reverse(score + score_advance), nx, ny, dir, path));
    }

    let mut seen = HashSet::new();
    for path in paths {
        seen.extend(path.into_iter());
    }

    seen.len()
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/16-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/16.txt")?;

    let mut grid = Vec::new();
    let mut sx = 0;
    let mut sy = 0;

    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            let cell = match c {
                '#' => Wall,
                '.' => Empty,
                'E' => Victory,
                'S' => {
                    sy = grid.len();
                    sx = row.len();
                    Empty
                }
                _ => unimplemented!("Unexpected character {c}"),
            };
            row.push(cell);
        }
        grid.push(row);
    }

    let score = path(&grid, sx, sy, East);
    println!("{score}");

    Ok(())
}
