use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/03.txt")?;
    let rx = Regex::new(r"[0-9]+")?;
    let mut s: i64 = 0;
    let input = input
        .lines()
        .map(|l| (l, l.chars().collect_vec()))
        .collect_vec();
    let mut symbols = HashMap::new();

    for (y, (_, chars)) in input.iter().enumerate() {
        for (x, c) in chars.iter().enumerate() {
            if c.is_ascii_digit() || *c == '.' {
                continue;
            }
            symbols.insert((x, y), *c);
        }
    }

    for (y, (line, _)) in input.iter().enumerate() {
        for m in rx.captures_iter(line) {
            let cap = m.get(0).unwrap();
            let x0 = cap.start() as i64;
            let num = cap.as_str();
            let x1 = (cap.start() + num.len()) as i64;
            let num = num.parse::<i64>().unwrap();
            let mut ok = false;

            let y = y as i64;
            for yy in (y - 1)..=(y + 1) {
                if yy < 0 {
                    continue;
                }
                let yy = yy as usize;

                for xx in (x0 - 1)..=x1 {
                    if xx < 0 {
                        continue;
                    }
                    let xx = xx as usize;
                    if symbols.contains_key(&(xx, yy)) {
                        ok = true;
                    }
                }
            }

            if ok {
                s += num;
            }
        }
    }

    println!("{s}");
    Ok(())
}
