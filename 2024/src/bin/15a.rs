use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Box,
    Wall,
}
use Cell::*;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/15-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/15.txt")?;

    let mut lines = input.lines();

    let mut grid = Vec::new();

    let mut rx = 0;
    let mut ry = 0;

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            let cell = match c {
                '#' => Wall,
                'O' => Box,
                '@' => {
                    rx = row.len() as i64;
                    ry = grid.len() as i64;
                    Empty
                }
                '.' => Empty,
                _ => unimplemented!("Unexpected char {c}"),
            };
            row.push(cell);
        }
        grid.push(row);
    }

    let mut moves = Vec::new();
    for line in lines {
        moves.extend(line.chars());
    }

    for action in moves {
        let (dx, dy) = match action {
            '^' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => unimplemented!("Unexpected action {action}"),
        };

        let mut empty = None;

        for i in 1.. {
            let x = rx + i * dx;
            let y = ry + i * dy;

            if x < 0 || y < 0 {
                break;
            }

            let Some(row) = grid.get(y as usize) else {
                break;
            };
            let Some(cell) = row.get(x as usize) else {
                break;
            };

            match cell {
                Box => {}
                Empty => {
                    empty = Some(i);
                    break;
                }
                Wall => break,
            };
        }

        if let Some(empty) = empty {
            for i in (2..=empty).rev() {
                let x1 = (rx + i * dx) as usize;
                let y1 = (ry + i * dy) as usize;
                let x0 = (rx + (i - 1) * dx) as usize;
                let y0 = (ry + (i - 1) * dy) as usize;

                (grid[y0][x0], grid[y1][x1]) = (grid[y1][x1], grid[y0][x0]);
            }

            grid[ry as usize][rx as usize] = Empty;
            rx += dx;
            ry += dy;
        }
    }

    let mut answer = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if matches!(cell, Box) {
                answer += x + 100 * y;
            }
        }
    }

    println!("{answer}");

    Ok(())
}
