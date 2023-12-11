use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/11.txt")?;
    let lines = input.lines().collect_vec();

    let omap = lines
        .iter()
        .flat_map(|line| {
            let row = line.chars().collect_vec();
            if line.contains('#') {
                vec![row]
            } else {
                vec![row.clone(), row]
            }
        })
        .collect_vec();

    let mut map = omap.iter().map(|_| vec![]).collect_vec();
    let mut stars = HashMap::new();
    let mut i = 0;

    for x in 0..omap[0].len() {
        let add = (0..omap.len()).all(|y| omap[y][x] == '.');
        for y in 0..omap.len() {
            let cell = omap[y][x];
            if cell == '#' {
                stars.insert((y, map[y].len()), i);
                i += 1;
            }

            map[y].push(omap[y][x]);
            if add {
                map[y].push('.');
            }
        }
    }

    let mut dist: i64 = 0;
    for ((ox, oy), oi) in stars.iter() {
        for ((tx, ty), ti) in stars.iter() {
            if ti <= oi {
                continue;
            }

            let dy = ((*ty as i64) - (*oy as i64)).abs();
            let dx = ((*tx as i64) - (*ox as i64)).abs();
            dist += dy + dx;
        }
    }

    println!("{dist}");
    Ok(())
}
