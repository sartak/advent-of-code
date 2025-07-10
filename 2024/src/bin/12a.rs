use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
enum Cell {
    Unprocessed(char),
    Alike,
    Processed,
}

fn floodfill(garden: &mut [Vec<Cell>], x: usize, y: usize, plant: char) -> (usize, usize) {
    let height = garden.len();
    let width = garden[0].len();

    let mut area = 0;
    let mut perimeter = 0;

    let mut queue = Vec::new();
    queue.push((x as i32, y as i32));

    while let Some((x, y)) = queue.pop() {
        let Some(cell) = garden[y as usize].get_mut(x as usize) else {
            unreachable!();
        };
        let c = match cell {
            Cell::Unprocessed(c) => c,
            Cell::Alike => continue,
            Cell::Processed => {
                perimeter += 1;
                continue;
            }
        };
        if *c != plant {
            perimeter += 1;
            continue;
        }

        area += 1;
        *cell = Cell::Alike;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = x + dx;
            let y = y + dy;

            if x < 0 || y < 0 {
                perimeter += 1;
                continue;
            }

            if x as usize >= width || y as usize >= height {
                perimeter += 1;
                continue;
            }

            queue.push((x, y));
        }
    }

    for line in garden.iter_mut() {
        for cell in line.iter_mut() {
            if matches!(cell, Cell::Alike) {
                *cell = Cell::Processed;
            }
        }
    }

    (area, perimeter)
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
            let (area, perimeter) = floodfill(&mut garden, x, y, c);
            answer += area * perimeter;
        }
    }

    println!("{answer}");

    Ok(())
}
