use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn process(cache: &mut HashMap<(u64, usize), u64>, stone: u64, blinks: usize) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(answer) = cache.get(&(stone, blinks)) {
        return *answer;
    }

    let answer = if stone == 0 {
        process(cache, 1, blinks - 1)
    } else {
        let s = stone.to_string();
        let halflen = s.len() >> 1;
        if halflen << 1 == s.len() {
            let a = s[..halflen].parse().unwrap();
            let b = s[halflen..].parse().unwrap();
            process(cache, a, blinks - 1) + process(cache, b, blinks - 1)
        } else {
            process(cache, stone * 2024, blinks - 1)
        }
    };

    cache.insert((stone, blinks), answer);

    answer
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/11-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/11.txt")?;

    let stones = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let mut cache = HashMap::new();
    let mut answer = 0;
    for stone in stones {
        answer += process(&mut cache, stone, 25);
    }

    println!("{answer}");

    Ok(())
}
