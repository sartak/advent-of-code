use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/11-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/11.txt")?;

    let mut stones = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    for _ in 0..25 {
        let mut new_stones = Vec::with_capacity(stones.len());

        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            let s = stone.to_string();
            let halflen = s.len() >> 1;
            if halflen << 1 == s.len() {
                let a = s[..halflen].parse().unwrap();
                let b = s[halflen..].parse().unwrap();
                new_stones.push(a);
                new_stones.push(b);
                continue;
            }

            new_stones.push(stone * 2024);
        }

        stones = new_stones;
    }

    println!("{}", stones.len());

    Ok(())
}
