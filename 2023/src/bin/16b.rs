use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

fn count(grid: &Vec<Vec<char>>, ox: i64, oy: i64, odir: Dir) -> i64 {
    let mut ans: i64 = 0;
    let mut beams = vec![(ox, oy, odir)];

    let mut energized = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect_vec())
        .collect_vec();
    let mut seen = HashSet::new();

    while !beams.is_empty() {
        let mut new_beams = vec![];
        for beam in beams {
            if seen.contains(&beam) {
                continue;
            }
            seen.insert(beam);

            let (mut x, mut y, mut dir) = beam;

            if let Some(ec) = energized.get_mut(y as usize) {
                if let Some(er) = ec.get_mut(x as usize) {
                    *er = true;
                }
            }

            match dir {
                Up => y -= 1,
                Down => y += 1,
                Left => x -= 1,
                Right => x += 1,
            };

            let Some(row) = grid.get(y as usize) else {
                continue;
            };
            let Some(cell) = row.get(x as usize) else {
                continue;
            };

            match cell {
                '.' => {}
                '/' => {
                    dir = match dir {
                        Up => Right,
                        Down => Left,
                        Left => Down,
                        Right => Up,
                    }
                }
                '\\' => {
                    dir = match dir {
                        Up => Left,
                        Down => Right,
                        Left => Up,
                        Right => Down,
                    }
                }
                '-' => {
                    if matches!(dir, Up | Down) {
                        dir = Left;
                        new_beams.push((x, y, Right));
                    }
                }
                '|' => {
                    if matches!(dir, Left | Right) {
                        dir = Up;
                        new_beams.push((x, y, Down));
                    }
                }
                _ => panic!(),
            };

            new_beams.push((x, y, dir));
        }

        beams = new_beams;
    }

    for row in energized {
        for cell in row {
            if cell {
                ans += 1;
            }
        }
    }

    ans
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/16.txt")?;
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let ymax = (0..grid.len())
        .map(|y| {
            let y = y as i64;
            let a = count(&grid, -1, y, Right);
            let b = count(&grid, grid[0].len() as i64, y, Left);
            a.max(b)
        })
        .max()
        .unwrap();

    let xmax = (0..grid[0].len())
        .map(|x| {
            let x = x as i64;
            let a = count(&grid, x, -1, Down);
            let b = count(&grid, x, grid.len() as i64, Up);
            a.max(b)
        })
        .max()
        .unwrap();

    let ans = ymax.max(xmax);

    println!("{ans}");
    Ok(())
}
