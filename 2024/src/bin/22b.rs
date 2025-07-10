use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/22-example.txt"
    } else {
        "input/22.txt"
    })?;

    let mut total = HashMap::new();

    for line in input.lines() {
        let mut n: u64 = line.parse().unwrap();

        let mut a;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;

        let mut seen = HashSet::new();

        for i in 0..=2000 {
            let orig = n % 10;

            let x = n * 64;
            n ^= x;
            n %= 16777216;

            let x = n / 32;
            n ^= x;
            n %= 16777216;

            let x = n * 2048;
            n ^= x;
            n %= 16777216;

            let value = (n % 10) as u8;

            a = b;
            b = c;
            c = d;
            d = value as i8 - (orig as i8);

            if i > 2 {
                let k = (a, b, c, d);
                if seen.insert(k) {
                    *total.entry(k).or_insert(0) += value as u64;
                }
            }
        }
    }

    println!("{}", total.values().max().unwrap());

    Ok(())
}
