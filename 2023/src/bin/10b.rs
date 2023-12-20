use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Dot,
    Gap,
    Start,
}
use Pipe::*;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};
impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            NS => '|',
            EW => '-',
            NE => 'L',
            NW => 'J',
            SW => '7',
            SE => 'F',
            Dot => '.',
            Gap => ' ',
            Start => 'S',
        };

        write!(f, "{c}")
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/10.txt")?;
    let lines = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut sx = 0;
    let mut sy = 0;
    let mut omap = vec![];
    for (y, line) in lines.iter().enumerate() {
        let mut row = vec![];
        for (x, c) in line.iter().enumerate() {
            let tile = match c {
                '|' => NS,
                '-' => EW,
                'L' => NE,
                'J' => NW,
                '7' => SW,
                'F' => SE,
                '.' | 'I' | 'O' => Dot,
                'S' => {
                    sx = x;
                    sy = y;
                    Start
                }
                _ => panic!(),
            };
            row.push(tile);
        }
        omap.push(row);
    }

    {
        let mut ds = vec![];
        let sx = sx as i64;
        let sy = sy as i64;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                if dy != 0 && dx != 0 {
                    continue;
                }

                let y = sy + dy;
                let x = sx + dx;
                if x < 0 || y < 0 {
                    continue;
                }

                if let Some(row) = omap.get(y as usize) {
                    if let Some(cell) = row.get(x as usize) {
                        let ok = match (cell, dx, dy) {
                            (NS, _, -1 | 1) => true,
                            (EW, 1 | -1, _) => true,
                            (NW, 1, 0) => true,
                            (NW, 0, 1) => true,
                            (SW, 1, 0) => true,
                            (SE, -1, 0) => true,
                            // incomplete
                            (_, _, _) => false,
                        };
                        if ok {
                            ds.push((dx, dy));
                        }
                    }
                }
            }
        }

        ds.sort();
        if matches!(&ds[..], [(0, 1), (1, 0)]) {
            omap[sy as usize][sx as usize] = SE;
        } else if matches!(&ds[..], [(0, -1), (1, 0)]) {
            omap[sy as usize][sx as usize] = NE;
        } else if matches!(&ds[..], [(-1, 0), (0, 1)]) {
            omap[sy as usize][sx as usize] = SW;
        } else {
            todo!("{ds:?}");
        }
    };

    let map = if true {
        let mut map = vec![];
        for row in omap {
            let mut line = vec![];
            for p in &row {
                line.push(*p);
                let c = match p {
                    NS => Gap,
                    EW => EW,
                    NE => EW,
                    NW => Gap,
                    SW => Gap,
                    SE => EW,
                    Dot => Gap,
                    Gap => unreachable!(),
                    Start => unreachable!(),
                };
                line.push(c);
            }
            map.push(line);

            let mut line = vec![];
            for p in &row {
                let (c, c2) = match p {
                    NS => (NS, Gap),
                    EW => (Gap, Gap),
                    NE => (Gap, Gap),
                    NW => (Gap, Gap),
                    SW => (NS, Gap),
                    SE => (NS, Gap),
                    Dot => (Gap, Gap),
                    Gap => (Gap, Gap),
                    Start => unreachable!(),
                };
                line.push(c);
                line.push(c2);
            }
            map.push(line);
        }
        sx *= 2;
        sy *= 2;
        map
    } else {
        omap
    };

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
            Gap => continue,
            Dot => continue,
            NS => vec![(0, -1), (0, 1)],
            EW => vec![(1, 0), (-1, 0)],
            NE => vec![(0, -1), (1, 0)],
            NW => vec![(0, -1), (-1, 0)],
            SW => vec![(0, 1), (-1, 0)],
            SE => vec![(0, 1), (1, 0)],
            Start => unreachable!(),
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

    let mut in_loop = map
        .iter()
        .map(|row| row.iter().map(|_| None).collect_vec())
        .collect_vec();
    loop {
        let mut sx = None;
        let mut sy = None;
        for (y, row) in map.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if in_loop[y][x].is_some() {
                    continue;
                }
                if seen.get(&(x as i64, y as i64)).is_none() {
                    sx = Some(x);
                    sy = Some(y);
                    break;
                }
            }
        }

        let Some(sx) = sx else { break };
        let Some(sy) = sy else { break };

        let mut enclosed = true;
        let mut touched = HashSet::new();
        let mut queue = vec![(sx, sy)];
        while let Some((x, y)) = queue.pop() {
            let k = (x, y);
            if touched.contains(&k) {
                continue;
            }
            touched.insert(k);

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if dx != 0 && dy != 0 {
                        continue;
                    }

                    let x = dx + x as i64;
                    let y = dy + y as i64;

                    // touched border
                    if x < 0 || y < 0 || x as usize >= map[0].len() || y as usize >= map.len() {
                        enclosed = false;
                        continue;
                    }

                    if let Some(row) = map.get(y as usize) {
                        if row.get(x as usize).is_some() {
                            if seen.get(&(x, y)).is_none() {
                                queue.push((x as usize, y as usize));
                            }
                        }
                    }
                }
            }
        }

        for (x, y) in touched {
            in_loop[y][x] = Some(enclosed);
        }
    }

    let mut count = 0;
    'y: for (y, row) in in_loop.iter().enumerate() {
        for (x, enclosed) in row.iter().enumerate() {
            if x % 2 == 1 {
                continue;
            } else if y % 2 == 1 {
                continue 'y;
            } else if matches!(enclosed, Some(true)) {
                count += 1;
            }
        }
    }
    println!("{count}");

    Ok(())
}
