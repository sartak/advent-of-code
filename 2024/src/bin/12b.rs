use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
enum Cell {
    Unprocessed(char),
    Alike,
    Processed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn as_delta(&self) -> (i32, i32) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

fn floodfill(garden: &mut [Vec<Cell>], x: usize, y: usize, plant: char) -> (usize, usize) {
    let height = garden.len();
    let width = garden[0].len();

    let mut area = 0;
    let mut perimeter = Vec::new();

    let mut queue = Vec::new();
    queue.push((x as i32, y as i32, None, x as i32, y as i32));

    while let Some((ox, oy, dir, fx, fy)) = queue.pop() {
        let Some(cell) = garden[oy as usize].get_mut(ox as usize) else {
            unreachable!();
        };
        let c = match cell {
            Cell::Unprocessed(c) => c,
            Cell::Alike => continue,
            Cell::Processed => {
                perimeter.push((fx, fy, dir.unwrap()));
                continue;
            }
        };
        if *c != plant {
            perimeter.push((fx, fy, dir.unwrap()));
            continue;
        }

        area += 1;
        *cell = Cell::Alike;

        for dir in [Up, Down, Left, Right] {
            let (dx, dy) = dir.as_delta();
            let x = ox + dx;
            let y = oy + dy;

            if x < 0 || y < 0 {
                perimeter.push((ox, oy, dir));
                continue;
            }

            if x as usize >= width || y as usize >= height {
                perimeter.push((ox, oy, dir));
                continue;
            }

            queue.push((x, y, Some(dir), ox, oy));
        }
    }

    let mut sides = 0;
    for dir in [Up, Down, Left, Right] {
        for (_, mut chunk) in &perimeter
            .iter()
            .filter_map(|(x, y, d)| {
                if *d == dir {
                    match dir {
                        Up | Down => Some((*y, *x)),
                        Left | Right => Some((*x, *y)),
                    }
                } else {
                    None
                }
            })
            .sorted()
            .chunk_by(|(major, _)| *major)
        {
            let Some((_, mut cur)) = chunk.next() else {
                unreachable!();
            };
            sides += 1;
            for (_, minor) in chunk {
                if minor != cur + 1 {
                    sides += 1;
                }
                cur = minor;
            }
        }
    }

    for line in garden.iter_mut() {
        for cell in line.iter_mut() {
            if matches!(cell, Cell::Alike) {
                *cell = Cell::Processed;
            }
        }
    }

    (area, sides)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/12-example.txt"
    } else {
        "input/12.txt"
    })?;

    let mut garden = input
        .lines()
        .map(|line| line.chars().map(Cell::Unprocessed).collect_vec())
        .collect_vec();
    let height = garden.len();
    let width = garden[0].len();

    let mut answer = 0;

    for y in 0..height {
        for x in 0..width {
            let cell = &garden[y][x];
            let c = match cell {
                Cell::Unprocessed(c) => *c,
                Cell::Alike => unreachable!(),
                Cell::Processed => continue,
            };
            let (area, sides) = floodfill(&mut garden, x, y, c);
            answer += area * sides;
        }
    }

    println!("{answer}");

    Ok(())
}
