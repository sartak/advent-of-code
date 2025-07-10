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
    fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
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

    let mut orig_px = 0;
    let mut orig_py = 0;
    let orig_dir = Up;

    let mut grid = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                orig_px = x;
                orig_py = y;
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

    let mut loops = 0;

    // @Performance: could run the simulation without modification, then try
    // placing obstacles only at the cells that the patrol touches, rather than
    // trying every cell. but this runs in 200ms on the problem input so it's ok
    for oy in 0..grid.len() {
        for ox in 0..grid[oy].len() {
            if ox == orig_px && oy == orig_py {
                continue;
            }

            let obs_row = grid.get_mut(oy).unwrap();
            let obs_cell = obs_row.get_mut(ox).unwrap();

            if obs_cell.obstacle {
                continue;
            }

            obs_cell.obstacle = true;
            let mut px = orig_px;
            let mut py = orig_py;
            let mut dir = orig_dir;

            loop {
                {
                    let Some(row) = grid.get_mut(py) else {
                        break;
                    };
                    let Some(cell) = row.get_mut(px) else {
                        break;
                    };
                    if cell.set_and_check_repeat(dir) {
                        loops += 1;
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

            let obs_row = grid.get_mut(oy).unwrap();
            let obs_cell = obs_row.get_mut(ox).unwrap();
            obs_cell.obstacle = false;

            for line in grid.iter_mut() {
                for cell in line {
                    cell.reset();
                }
            }
        }
    }

    println!("{loops}");

    Ok(())
}
