use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i64, i64) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }

    fn rotate(&self) -> Self {
        match self {
            Up => Right,
            Down => Left,
            Right => Down,
            Left => Up,
        }
    }
}

#[derive(Debug)]
struct Cell {
    obstacle: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

use Direction::*;

impl Cell {
    fn seen(&self) -> bool {
        self.up || self.down || self.left || self.right
    }

    fn set_and_check_repeat(&mut self, dir: Direction) -> bool {
        let is_repeat = match dir {
            Up => self.up,
            Down => self.down,
            Left => self.left,
            Right => self.right,
        };

        match dir {
            Up => self.up = true,
            Down => self.down = true,
            Left => self.left = true,
            Right => self.right = true,
        };

        is_repeat
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/06-example.txt"
    } else {
        "input/06.txt"
    })?;

    let mut px = 0;
    let mut py = 0;
    let mut dir = Up;

    let mut grid = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                px = x;
                py = y;
            }

            row.push(Cell {
                obstacle: c == '#',
                up: false,
                down: false,
                left: false,
                right: false,
            });
        }

        grid.push(row);
    }

    let mut seen = 0;

    loop {
        {
            let Some(row) = grid.get_mut(py) else {
                break;
            };
            let Some(cell) = row.get_mut(px) else {
                break;
            };
            if !cell.seen() {
                seen += 1;
            }
            if cell.set_and_check_repeat(dir) {
                break;
            }
        }

        {
            let (dx, dy) = dir.delta();
            let Ok(nx) = usize::try_from(px as i64 + dx) else {
                break;
            };
            let Ok(ny) = usize::try_from(py as i64 + dy) else {
                break;
            };

            let Some(row) = grid.get(ny) else {
                break;
            };
            let Some(cell) = row.get(nx) else {
                break;
            };

            if cell.obstacle {
                dir = dir.rotate();
            } else {
                px = nx;
                py = ny;
            }
        }
    }

    println!("{seen}");

    Ok(())
}
