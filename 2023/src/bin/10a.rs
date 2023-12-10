use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
}
use Pipe::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/10.txt")?;
    let lines = input.lines().collect_vec();

    let mut sx = 0;
    let mut sy = 0;
    let mut map = vec![];
    for (y, line) in lines.iter().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '|' => Some(NS),
                '-' => Some(EW),
                'L' => Some(NE),
                'J' => Some(NW),
                '7' => Some(SW),
                'F' => Some(SE),
                '.' => None,
                'S' => {
                    sx = x;
                    sy = y;
                    Some(Start)
                }
                _ => panic!(),
            };
            row.push(tile);
        }
        map.push(row);
    }

    let mut max = 0;
    let mut seen = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((sx as i64, sy as i64, 0));
    while let Some((x, y, d)) = queue.pop_front() {
        if seen.contains_key(&(x, y)) {
            continue;
        }
        seen.insert((x, y), d);

        if d > max {
            max = d;
        }

        let deltas: Vec<(i64, i64)> = match map[y as usize][x as usize] {
            None => continue,
            Some(NS) => vec![(0, -1), (0, 1)],
            Some(EW) => vec![(1, 0), (-1, 0)],
            Some(NE) => vec![(0, -1), (1, 0)],
            Some(NW) => vec![(0, -1), (-1, 0)],
            Some(SW) => vec![(0, 1), (-1, 0)],
            Some(SE) => vec![(0, 1), (1, 0)],
            Some(Start) => {
                let mut ds = vec![];
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        if dy != 0 && dx != 0 {
                            continue;
                        }

                        let Some(y) = y.checked_add(dy) else { continue };
                        let Some(x) = x.checked_add(dx) else { continue };
                        let y = y as usize;
                        let x = x as usize;

                        if let Some(row) = map.get(y) {
                            if let Some(cell) = row.get(x) {
                                let ok = match (cell, dx, dy) {
                                    (None, _, _) => false,
                                    (Some(NS), _, -1 | 1) => true,
                                    (Some(EW), 1 | -1, _) => true,
                                    (Some(NW), 1, 0) => true,
                                    (Some(SW), 1, 0) => true,
                                    /*
                                    (Some(SW), ?, ?) => true,
                                    (Some(SE), ?, ?) => true,
                                    (Some(SE), ?, ?) => true,
                                    (Some(NW), 0, -1) => true,
                                    (Some(NE), ?, ?) => true,
                                    (Some(NE), ?, ?) => true,
                                    */
                                    (_, _, _) => false,
                                };
                                if ok {
                                    ds.push((dx, dy));
                                }
                            }
                        }
                    }
                }
                ds
            }
        };

        for (dx, dy) in deltas {
            let Some(y) = y.checked_add(dy) else { continue };
            let Some(x) = x.checked_add(dx) else { continue };
            let y = y as usize;
            let x = x as usize;

            if let Some(row) = map.get(y) {
                if row.get(x).is_some() {
                    queue.push_back((x as i64, y as i64, d + 1));
                }
            }
        }
    }

    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if let Some(d) = seen.get(&(x as i64, y as i64)) {
                if *d >= 10 {
                    print!("*");
                } else {
                    print!("{d}");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{max}");
    Ok(())
}
