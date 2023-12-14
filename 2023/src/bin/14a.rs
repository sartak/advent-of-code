use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/14.txt")?;
    let mut map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut ans: i64 = 0;

    let yy = map.len();
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
