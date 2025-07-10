use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    BoxLeft,
    BoxRight,
    Wall,
}
use Cell::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/15-example.txt"
    } else {
        "input/15.txt"
    })?;

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
            let (left, right) = match c {
                '#' => (Wall, Wall),
                'O' => (BoxLeft, BoxRight),
                '@' => {
                    rx = row.len() as i64;
                    ry = grid.len() as i64;
                    (Empty, Empty)
                }
                '.' => (Empty, Empty),
                _ => unimplemented!("Unexpected char {c}"),
            };
            row.push(left);
            row.push(right);
        }
        grid.push(row);
    }

    let mut moves = Vec::new();
    for line in lines {
        moves.extend(line.chars());
    }

    'action: for action in moves {
        let (dx, dy) = match action {
            '^' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => unimplemented!("Unexpected action {action}"),
        };

        if dy == 0 {
            let mut empty = None;

            for i in 1.. {
                let x = rx + i * dx;

                if x < 0 {
                    break;
                }

                let Some(row) = grid.get(ry as usize) else {
                    break;
                };
                let Some(cell) = row.get(x as usize) else {
                    break;
                };

                match cell {
                    BoxLeft => {}
                    BoxRight => {}
                    Empty => {
                        empty = Some(i);
                        break;
                    }
                    Wall => break,
                };
            }

            if let Some(empty) = empty {
                let line = grid.get_mut(ry as usize).unwrap();
                for i in (2..=empty).rev() {
                    let x1 = (rx + i * dx) as usize;
                    let x0 = (rx + (i - 1) * dx) as usize;

                    (line[x0], line[x1]) = (line[x1], line[x0]);
                }

                line[rx as usize] = Empty;
                rx += dx;
            }
        } else {
            let mut frontier = vec![rx];
            let mut boxes = Vec::new();

            for i in 1.. {
                let mut new_frontier = Vec::new();

                let y = ry + i * dy;
                if y < 0 {
                    break;
                }
                if y as usize >= grid.len() {
                    break;
                }

                for x in frontier {
                    let cell = grid[y as usize][x as usize];
                    match cell {
                        BoxLeft => {
                            if new_frontier.contains(&x) {
                                continue;
                            }
                            new_frontier.push(x);
                            new_frontier.push(x + 1);
                            boxes.push((y, x, x + 1));
                        }
                        BoxRight => {
                            if new_frontier.contains(&x) {
                                continue;
                            }
                            new_frontier.push(x - 1);
                            new_frontier.push(x);
                            boxes.push((y, x - 1, x));
                        }
                        Empty => {}
                        Wall => {
                            continue 'action;
                        }
                    }
                }

                if new_frontier.is_empty() {
                    break;
                }
                frontier = new_frontier;
            }

            for (y, xl, xr) in boxes.into_iter().rev() {
                let y0 = y as usize;
                let y1 = (y + dy) as usize;
                let xl = xl as usize;
                let xr = xr as usize;
                (grid[y0][xl], grid[y1][xl]) = (grid[y1][xl], grid[y0][xl]);
                (grid[y0][xr], grid[y1][xr]) = (grid[y1][xr], grid[y0][xr]);
            }

            ry += dy;
        }
    }

    let mut answer = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if matches!(cell, BoxLeft) {
                answer += x + 100 * y;
            }
        }
    }

    println!("{answer}");

    Ok(())
}
