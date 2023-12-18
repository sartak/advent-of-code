use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/18.txt")?;
    let lines = input.lines().collect_vec();
    let rx = Regex::new(r"^(\w) (\d+) \(\#(\w+)\)$")?;

    let mut ans: i64 = 0;
    let ox = 256i64;
    let oy = 256i64;
    let mut x = ox;
    let mut y = oy;
    let mut mx = 0;
    let mut my = 0;

    let mut grid = vec![vec![0; 512]; 512];

    grid[y as usize][x as usize] = 1;

    lines.iter().for_each(|&line| {
        let caps = rx.captures(line).unwrap();
        let dir = caps.get(1).unwrap().as_str();
        let steps = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let mut dx = 0;
        let mut dy = 0;
        match dir {
            "R" => dx = steps,
            "L" => dx = -steps,
            "U" => dy = -steps,
            "D" => dy = steps,
            _ => panic!(),
        };

        for xx in 0..=dx.abs() {
            let x = (x + xx * dx.signum()) as usize;
            for yy in 0..=dy.abs() {
                let y = (y + yy * dy.signum()) as usize;
                grid[y][x] = 1;
            }
        }

        x += dx;
        y += dy;

        mx = mx.max(x);
        my = my.max(x);
    });

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((0, 0));
    while let Some((x, y)) = queue.pop_front() {
        let k = (x, y);
        if seen.contains(&k) {
            continue;
        }
        seen.insert(k);

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                if dy != 0 && dx != 0 {
                    continue;
                }

                if x + dx < 0 || y + dy < 0 {
                    continue;
                }

                let x = x + dx;
                let y = y + dy;

                if let Some(row) = grid.get_mut(y as usize) {
                    if let Some(cell) = row.get_mut(x as usize) {
                        if *cell == 1 {
                            continue;
                        } else {
                            *cell = -1;
                            queue.push_back((x, y));
                        }
                    }
                }
            }
        }
    }

    for row in grid {
        for cell in row {
            if cell == 1 || cell == 0 {
                ans += 1;
            }
        }
    }

    println!("{ans}");
    Ok(())
}
