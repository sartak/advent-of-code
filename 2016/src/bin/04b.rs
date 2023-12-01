use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/04.txt")?;

    input
        .lines()
        .filter_map(|line| {
            let (name, checksum) = line.split_once('[').unwrap();
            let (name, sector) = name.rsplit_once('-').unwrap();
            let checksum = checksum.strip_suffix(']').unwrap();

            let mut counts = HashMap::new();
            for c in name.chars() {
                counts.entry(c).and_modify(|c| *c += 1).or_insert(1);
            }
            let got = counts
                .into_iter()
                .filter(|(k, _)| k.is_ascii_lowercase())
                .sorted_by_key(|&(k, v)| (-v, k))
                .take(5)
                .map(|(k, _)| k)
                .collect::<String>();

            if checksum != got {
                return None;
            }

            let sector = sector.parse::<i32>().unwrap();
            let name = name
                .split('-')
                .map(|word| {
                    word.chars()
                        .map(|c| ((((c as u8 - b'a') as i32 + sector) % 26) as u8 + b'a') as char)
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join(" ");
            Some((sector, name))
        })
        .filter(|(_, name)| name.to_lowercase().contains("north"))
        .for_each(|(sector, name)| println!("{sector} {name}"));

    Ok(())
}
