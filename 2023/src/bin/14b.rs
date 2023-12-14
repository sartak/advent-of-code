use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/14.txt")?;
    let mut map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut ans: i64 = 0;

    let yy = map.len();
    let xx = map[0].len();
    let mut seen = HashMap::new();
    let mut cycle = 0i64;
    let mut found = false;
    let max = 1000000000;

    while cycle < max {
        cycle += 1;

        // north
        loop {
            let mut changed = false;

            for x in 0..map[0].len() {
                for y in 0..map.len() {
                    if map[y][x] != '.' {
                        continue;
                    }
                    if y + 1 < yy && map[y + 1][x] == 'O' {
                        map[y][x] = 'O';
                        map[y + 1][x] = '.';
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        // west
        loop {
            let mut changed = false;

            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] != '.' {
                        continue;
                    }
                    if x + 1 < xx && map[y][x + 1] == 'O' {
                        map[y][x] = 'O';
                        map[y][x + 1] = '.';
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        // south
        loop {
            let mut changed = false;

            for x in 0..map[0].len() {
                for y in (0..map.len()).rev() {
                    if map[y][x] != '.' {
                        continue;
                    }
                    if y > 0 && map[y - 1][x] == 'O' {
                        map[y][x] = 'O';
                        map[y - 1][x] = '.';
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        // east
        loop {
            let mut changed = false;

            for y in 0..map.len() {
                for x in (0..map[0].len()).rev() {
                    if map[y][x] != '.' {
                        continue;
                    }
                    if x > 0 && map[y][x - 1] == 'O' {
                        map[y][x] = 'O';
                        map[y][x - 1] = '.';
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        if !found {
            let mut key = vec![];
            for y in 0..map.len() {
                for x in (0..map[0].len()).rev() {
                    if map[y][x] == 'O' {
                        key.push(format!("{x},{y}"));
                    }
                }
            }
            let key = key.join("|");

            if let Some(prev) = seen.get(&key) {
                found = true;

                let gap = cycle - prev;
                while cycle + gap < max {
                    cycle += gap;
                }
            }

            seen.insert(key, cycle);
        }
    }

    /*
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
    */

    for (y, row) in map.into_iter().enumerate() {
        for cell in row.into_iter() {
            if cell == 'O' {
                ans += (yy - y) as i64;
            }
        }
    }

    println!("{ans}");
    Ok(())
}
