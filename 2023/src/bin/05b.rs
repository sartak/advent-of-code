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
        .tuples()
        .map(|(start, len)| start..(start + len))
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
                .flat_map(|range| {
                    transform
                        .overlapping(&range)
                        .map(|(overlap, offset)| {
                            let start = overlap.start.max(range.start) + offset;
                            let end = overlap.end.min(range.end) + offset;
                            start..end
                        })
                        .chain(transform.gaps(&range))
                        .collect_vec()
                })
                .collect_vec()
        })
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap();

    println!("{min}");

    Ok(())
}
