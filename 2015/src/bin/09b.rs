use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut indexes: HashMap<String, usize> = HashMap::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

    let mut i = 0;
    let input = std::fs::read_to_string("input/09.txt")?;
    for line in input.lines() {
        let Some((path, dist)) = line.split_once(" = ") else {
            unreachable!()
        };

        let Some((from, to)) = path.split_once(" to ") else {
            unreachable!()
        };

        let from = *indexes.entry(String::from(from)).or_insert_with(|| {
            i += 1;
            i
        });
        let to = *indexes.entry(String::from(to)).or_insert_with(|| {
            i += 1;
            i
        });

        let dist = dist.parse()?;
        distances.insert((from, to), dist);
        distances.insert((to, from), dist);
    }

    let res = indexes
        .values()
        .permutations(indexes.len())
        .map(|path| {
            path.into_iter()
                .tuple_windows()
                .map(|(from, to)| distances.get(&(*from, *to)).unwrap())
                .sum::<usize>()
        })
        .max();

    let Some(res) = res else { unreachable!() };

    println!("{res}");

    Ok(())
}
