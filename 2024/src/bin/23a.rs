use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/23-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/23.txt")?;

    let mut cx: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split('-').collect_tuple().unwrap();
        for (a, b) in [(a, b), (b, a)] {
            cx.entry(a).or_default().push(b);
        }
    }

    let mut sets = Vec::new();
    for (a, a_cx) in &cx {
        for b in a_cx {
            if b < a {
                continue;
            }

            let b_cx = cx.get(b).unwrap();
            for c in b_cx {
                if c < b {
                    continue;
                }

                if a == c {
                    continue;
                }

                if a_cx.contains(c)
                    && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                {
                    sets.push((a, b, c));
                }
            }
        }
    }

    println!("{}", sets.len());

    Ok(())
}
