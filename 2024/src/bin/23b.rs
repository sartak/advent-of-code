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

    let mut queue = cx
        .iter()
        .map(|(&a, cx)| (vec![a], cx.clone()))
        .collect_vec();

    let mut best = Vec::new();
    'eval: while let Some((mut inside, mut candidates)) = queue.pop() {
        let Some(candidate) = candidates.pop() else {
            if inside.len() > best.len() {
                best = inside;
            }
            continue;
        };

        queue.push((inside.clone(), candidates.clone()));

        let others = cx.get(candidate).unwrap();
        for join in &inside {
            if !others.contains(join) {
                continue 'eval;
            }
        }

        inside.push(candidate);
        queue.push((inside, candidates));
    }

    best.sort();
    println!("{}", best.join(","));

    Ok(())
}
