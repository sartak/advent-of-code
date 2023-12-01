use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/19.txt")?;
    let mut lines = input.lines().collect::<Vec<_>>();
    let start = String::from(lines.pop().unwrap());
    let replacements = lines
        .into_iter()
        .filter_map(|line| line.split_once(" => "))
        .collect::<Vec<_>>();
    let target = String::from("e");

    let mut seen = HashSet::new();
    let mut stack = vec![(start, 0)];

    let res = 'main: loop {
        let (molecule, count) = stack.pop().unwrap();
        for start in 0..molecule.len() {
            let prefix = &molecule[..start];
            let rest = &molecule[start..];
            for (replacement, pattern) in &replacements {
                if rest.starts_with(pattern) {
                    let suffix = rest.replacen(pattern, replacement, 1);
                    let new = format!("{prefix}{suffix}");
                    if new == target {
                        break 'main count + 1;
                    }

                    if !seen.contains(&new) {
                        seen.insert(new.clone());
                        stack.push((new, count + 1));
                    }
                }
            }
        }
    };

    println!("{res}");

    Ok(())
}
