use anyhow::Result;

enum Direction {
    North,
    South,
    East,
    West,
}
use Direction::*;

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
    }

    fn turn(&self, left: bool) -> Self {
        match self {
            North => {
                if left {
                    West
                } else {
                    East
                }
            }
            South => {
                if left {
                    East
                } else {
                    West
                }
            }
            East => {
                if left {
                    North
                } else {
                    South
                }
            }
            West => {
                if left {
                    South
                } else {
                    North
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/01.txt")?;

    let mut direction = North;
    let mut x = 0;
    let mut y = 0;

    for movement in input.split(", ") {
        let (dir, amount) = movement.trim().split_at(1);
        let amount = amount.parse::<i32>()?;
        direction = direction.turn(dir == "L");
        let (dx, dy) = direction.delta();
        x += dx * amount;
        y += dy * amount;
    }

    let res = x.abs() + y.abs();

    println!("{res}");
    Ok(())
}
