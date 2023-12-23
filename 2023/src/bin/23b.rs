use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/23.txt")?;
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let sy = 0usize;
    let sx = grid[0].iter().position(|&c| c == '.').unwrap();
    let ty = grid.len() - 1;
    let tx = grid[ty].iter().position(|&c| c == '.').unwrap();

    let mut nodes = vec![(sx, sy), (tx, ty)];
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] == '#' {
                continue;
            }

            let mut count = 0;
            for dy in -1..=1 {
                if y as i64 + dy < 0 {
                    continue;
                }
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    if dx != 0 && dy != 0 {
                        continue;
                    }

                    if x as i64 + dx < 0 {
                        continue;
                    }

                    let y = (y as i64 + dy) as usize;
                    let x = (x as i64 + dx) as usize;
                    if let Some(row) = grid.get(y) {
                        if let Some(&cell) = row.get(x) {
                            if cell == '#' {
                                continue;
                            }

                            count += 1;
                        }
                    }
                }
            }

            if count > 2 {
                nodes.push((x, y));
            }
        }
    }

    let is_node = nodes.iter().collect::<HashSet<_>>();

    let mut edges = HashMap::new();
    for &(sx, sy) in &nodes {
        let mut queue = VecDeque::new();
        queue.push_back((sx, sy, vec![(sx, sy)]));

        let mut reachable = HashMap::new();
        while let Some((x, y, path)) = queue.pop_front() {
            if !(x == sx && y == sy) && is_node.contains(&(x, y)) {
                assert!(!reachable.contains_key(&(x, y)));
                reachable.insert((x, y), path.len() - 1);
                continue;
            }

            for dy in -1..=1 {
                if y as i64 + dy < 0 {
                    continue;
                }
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    if dx != 0 && dy != 0 {
                        continue;
                    }

                    if x as i64 + dx < 0 {
                        continue;
                    }

                    let y = (y as i64 + dy) as usize;
                    let x = (x as i64 + dx) as usize;
                    if let Some(row) = grid.get(y) {
                        if let Some(&cell) = row.get(x) {
                            if cell == '#' {
                                continue;
                            }

                            if path.contains(&(x, y)) {
                                continue;
                            }

                            let mut path = path.clone();
                            path.push((x, y));
                            queue.push_back((x, y, path));
                        }
                    }
                }
            }
        }

        edges.insert((sx, sy), reachable);
    }

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy, 0, vec![(sx, sy)]));

    let mut max = 0;
    while let Some((x, y, cost, path)) = queue.pop_front() {
        if x == tx && y == ty && cost > max {
            max = cost;
        }

        let reachable = edges.get(&(x, y)).unwrap();
        for ((nx, ny), nc) in reachable {
            if path.contains(&(*nx, *ny)) {
                continue;
            }

            let mut path = path.clone();
            path.push((*nx, *ny));
            queue.push_back((*nx, *ny, cost + *nc, path));
        }
    }

    /*
    for (y, row) in grid.iter().en() {
        println!();
        for (x, cell) in row.iter().en() {
            if max_path.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{cell}");
            }
        }
    }
    */

    println!("{}", max);

    Ok(())
}
