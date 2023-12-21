use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/21.txt")?;
    let grid = input
        .lines()
        .map(|row| row.chars().collect_vec())
        .collect_vec();
    let mut sx = 0;
    let mut sy = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                sx = x;
                sy = y;
                break;
            }
        }
    }

    let yy = grid.len();
    let xx = grid[0].len();

    /*
    let spots: usize = grid
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&&cell| cell == '.' || cell == 'S')
                .count()
        })
        .sum();
    */

    //let dists = HashMap::new();
    //let pd = 1;
    //let mut p1ok = HashSet::new();

    let actual = 26501365;
    let start = actual % xx;
    for target in (start..).step_by(xx) {
        let mut queue = VecDeque::new();
        queue.push_back((sx as i64, sy as i64, 0));

        let mut seen = HashSet::new();
        //let mut p0 = HashSet::new();
        let mut memo = HashSet::new();
        while let Some((x, y, d)) = queue.pop_front() {
            if d == target {
                /*
                let px = x / xx as i64;
                let py = y / yy as i64;
                if x >= 0 && y >= 0 && px == 0 && py == 0 {
                    p0.insert((x, y));
                }
                */

                seen.insert((x, y));
                continue;
            }

            let k = (x, y, d);
            if memo.contains(&k) {
                continue;
            }
            memo.insert(k);

            for dy in -1i64..=1 {
                let y = y + dy;
                for dx in -1i64..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if dx != 0 && dy != 0 {
                        continue;
                    }

                    let x = x + dx;

                    /*
                    let px = x / xx as i64;
                    let py = y / yy as i64;
                    if px.abs() > pd || py.abs() > pd {
                        continue;
                    }
                    if px.abs() <= 1 && py.abs() <= 1 {
                        p1ok.insert((x, y));
                    }
                    */

                    let cell =
                        grid[y.rem_euclid(yy as i64) as usize][x.rem_euclid(xx as i64) as usize];
                    if cell == '.' || cell == 'S' {
                        queue.push_back((x, y, d + 1));
                    }
                }
            }
        }
        /*

        let p1 = seen
            .iter()
            .filter(|(x, y)| p1ok.contains(&(*x, *y)))
            .count();
        */
        eprintln!("{target},{}", seen.len());
        /*
        println!();
        for py in -pd..=pd {
            for (y, row) in grid.iter().en() {
                for px in -pd..=pd {
                    if py == 0 && px == 0 {
                        print!("\x1b[1;37m");
                    } else if py.abs() == 2 || px.abs() == 2 {
                        print!("\x1b[1;30m");
                    }
                    for (x, &cell) in row.iter().en() {
                        if seen.contains(&(x as i64 + px * xx as i64, y as i64 + py * yy as i64)) {
                            print!("O");
                        } else {
                            if cell == 'S' && (px != 0 || py != 0) {
                                print!(".");
                            } else {
                                print!("{cell}");
                            }
                        }
                    }
                    print!("\x1b[m");
                }
                println!();
            }
        }
        */
    }

    Ok(())
}
