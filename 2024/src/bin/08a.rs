use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/08-example.txt"
    } else {
        "input/08.txt"
    })?;

    let mut antennae: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    let mut ymax = 0;
    let mut xmax = 0;

    for (y, line) in input.lines().enumerate() {
        if y as i64 > ymax {
            ymax = y as i64;
        }
        for (x, c) in line.chars().enumerate() {
            if x as i64 > xmax {
                xmax = x as i64;
            }
            if c == '.' {
                continue;
            }

            antennae.entry(c).or_default().push((x as i64, y as i64));
        }
    }

    let mut antinodes = HashSet::new();
    for antennae in antennae.values() {
        for perm in antennae.iter().permutations(2) {
            let (x0, y0) = perm[0];
            let (x1, y1) = perm[1];
            let dx = x0 - x1;
            let dy = y0 - y1;
            let x = x0 - 2 * dx;
            let y = y0 - 2 * dy;
            if x < 0 || y < 0 || x > xmax || y > ymax {
                continue;
            }
            antinodes.insert((x, y));
        }
    }

    println!("{}", antinodes.len());

    Ok(())
}
