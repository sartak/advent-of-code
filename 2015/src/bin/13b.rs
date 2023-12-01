use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut guests = HashMap::new();
    let mut happiness = HashMap::new();
    let mut i = 0;

    let input = std::fs::read_to_string("input/13.txt")?;
    let regex =
        Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.$")?;
    for line in input.lines() {
        let Some(caps) = regex.captures(line) else {
            unreachable!()
        };
        let (_, [a, action, amount, b]) = caps.extract();
        let amount: i32 = if action == "gain" {
            amount.parse()?
        } else {
            -amount.parse()?
        };

        let a = *guests.entry(String::from(a)).or_insert_with(|| {
            i += 1;
            i
        });
        let b = *guests.entry(String::from(b)).or_insert_with(|| {
            i += 1;
            i
        });

        for k in [(a, b), (b, a)] {
            happiness
                .entry(k)
                .and_modify(|v| *v += amount)
                .or_insert(amount);
        }
    }

    let me = i + 1;
    for guest in guests.values() {
        for k in [(me, *guest), (*guest, me)] {
            happiness.insert(k, 0);
        }
    }
    guests.insert(String::from(""), me);

    let res = guests
        .values()
        .permutations(guests.len())
        .map(|path| {
            path.into_iter()
                .circular_tuple_windows()
                .map(|(a, b)| happiness.get(&(*a, *b)).unwrap())
                .sum::<i32>()
        })
        .max();

    let Some(res) = res else { unreachable!() };

    println!("{res}");

    Ok(())
}
