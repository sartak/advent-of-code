use anyhow::Result;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}
use Dir::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/17.txt")?;
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut ans = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0, 0, East, 0)));

    let xx = map[0].len();
    let yy = map.len();
    let mut seen = HashSet::new();

    while let Some(Reverse((heat, x, y, dir, consec))) = queue.pop() {
        let k = (x, y, dir, consec);
        if seen.contains(&k) {
            continue;
        }
        seen.insert(k);

        if x == xx - 1 && y == yy - 1 {
            ans = heat;
            break;
        }

        let x = x as i64
            + match dir {
                West => -1,
                East => 1,
                _ => 0,
            };

        let y = y as i64
            + match dir {
                North => -1,
                South => 1,
                _ => 0,
            };

        if x < 0 || y < 0 {
            continue;
        }

        let x = x as usize;
        let y = y as usize;

        let heat = if let Some(row) = map.get(y) {
            if let Some(cell) = row.get(x) {
                heat + cell.to_digit(10).unwrap()
            } else {
                continue;
            }
        } else {
            continue;
        };

        let left = match dir {
            North => West,
            South => East,
            East => North,
            West => South,
        };

        let right = match dir {
            North => East,
            South => West,
            East => South,
            West => North,
        };

        queue.push(Reverse((heat, x, y, left, 0)));
        queue.push(Reverse((heat, x, y, right, 0)));
        if consec < 2 {
            queue.push(Reverse((heat, x, y, dir, consec + 1)));
        }
    }

    println!("{ans}");
    Ok(())
}
