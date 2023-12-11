use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/11.txt")?;
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut my = map.iter().map(|_| false).collect_vec();
    let mut mx = map[0].iter().map(|_| false).collect_vec();

    for (y, row) in map.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            my[y] = true;
        }
    }

    for x in 0..map[0].len() {
        if (0..map.len()).all(|y| map[y][x] == '.') {
            mx[x] = true;
        }
    }

    let mut stars = HashMap::new();
    let mut i = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '#' {
                stars.insert((x, y), i);
                i += 1;
            }
        }
    }

    let dilation = 1000000 - 1;
    let mut dist: i64 = 0;
    for ((ox, oy), oi) in stars.iter() {
        for ((tx, ty), ti) in stars.iter() {
            if ti <= oi {
                continue;
            }

            let (&ax, &bx) = if tx < ox { (tx, ox) } else { (ox, tx) };
            let (&ay, &by) = if ty < oy { (ty, oy) } else { (oy, ty) };
            for x in (ax + 1)..bx {
                if mx[x] {
                    dist += dilation;
                }
            }
            for y in (ay + 1)..by {
                if my[y] {
                    dist += dilation;
                }
            }
            let dy = ((*ty as i64) - (*oy as i64)).abs();
            let dx = ((*tx as i64) - (*ox as i64)).abs();
            dist += dy + dx;
        }
    }

    println!("{dist}");
    Ok(())
}
