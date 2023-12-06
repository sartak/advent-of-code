use anyhow::Result;
use itertools::Itertools;
use rangemap::RangeMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/05.txt")?;
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();

    let _empty = lines.next();

    let min = lines
        .batching(|lines| {
            let _header = lines.next();

            let transform = lines
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let (dest, source, len) = line
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap())
                        .tuples()
                        .next()
                        .unwrap();
                    (source..(source + len), dest - source)
                })
                .collect::<RangeMap<_, _>>();
            if transform.is_empty() {
                None
            } else {
                Some(transform)
            }
        })
        .fold(seeds, |nums, transform| {
            nums.into_iter()
                .map(|num| num + transform.get(&num).unwrap_or(&0))
                .collect_vec()
        })
        .into_iter()
        .min()
        .unwrap();

    println!("{min}");

    Ok(())
}
