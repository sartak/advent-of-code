use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/03.txt")?;
    let input = input.lines().collect_vec();
    let rx = Regex::new(r"[0-9]+")?;

    let mut gears = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '*' {
                    Some(((x, y), Vec::new()))
                } else {
                    None
                }
            })
        })
        .collect::<HashMap<_, _>>();

    for (y, line) in input.into_iter().enumerate() {
        for res in rx.find_iter(line) {
            let num = res.as_str();
            let x0 = res.start();
            let x1 = x0 + num.len();
            let num = num.parse::<usize>().unwrap();

            for (x, y) in
                (x0.saturating_sub(1)..=x1).cartesian_product(y.saturating_sub(1)..=(y + 1))
            {
                if let Some(nums) = gears.get_mut(&(x, y)) {
                    nums.push(num);
                }
            }
        }
    }

    let sum: usize = gears
        .values()
        .filter_map(|nums| {
            if nums.len() == 2 {
                Some(nums[0] * nums[1])
            } else {
                None
            }
        })
        .sum();

    println!("{sum}");
    Ok(())
}
