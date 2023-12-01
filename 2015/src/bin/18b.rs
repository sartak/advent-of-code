use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/18.txt")?;
    let mut a: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let mut b = a.clone();
    let bottom = a.len() - 1;
    let right = a[0].len() - 1;
    let steps = 100;

    for i in 0..steps {
        let (prev, next) = if i % 2 == 0 {
            (&a, &mut b)
        } else {
            (&b, &mut a)
        };

        for (y, row) in prev.iter().enumerate() {
            for (x, &on) in row.iter().enumerate() {
                let count = (-1..=1)
                    .cartesian_product(-1..=1)
                    .filter(|&(dx, dy): &(i32, i32)| {
                        if dx == 0 && dy == 0 {
                            false
                        } else {
                            if let Ok(x) = usize::try_from(x as i32 + dx) {
                                if let Ok(y) = usize::try_from(y as i32 + dy) {
                                    if let Some(row) = prev.get(y) {
                                        if let Some(cell) = row.get(x) {
                                            return *cell;
                                        }
                                    }
                                }
                            }

                            false
                        }
                    })
                    .count();

                let alive = if on {
                    count == 2 || count == 3
                } else {
                    count == 3
                };

                next[y][x] = alive;
            }
        }

        next[0][0] = true;
        next[bottom][0] = true;
        next[0][right] = true;
        next[bottom][right] = true;
    }

    let board = if steps % 2 == 0 { a } else { b };

    /*
    for row in &board {
        for cell in row {
            print!("{}", if *cell { "#" } else { "." });
        }
        println!("");
    }
    */

    let alive: usize = board
        .into_iter()
        .map(|line| line.into_iter().filter(|cell| *cell).count())
        .sum();

    println!("{alive}");

    Ok(())
}
