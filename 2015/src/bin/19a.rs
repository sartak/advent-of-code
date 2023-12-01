use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/19.txt")?;
    let mut lines = input.lines().collect::<Vec<_>>();
    let molecule = lines.pop().unwrap();
    let replacements = lines
        .into_iter()
        .filter_map(|line| line.split_once(" => "))
        .collect::<Vec<_>>();

    let mut seen = HashSet::new();
    for start in 0..molecule.len() {
        let prefix = &molecule[..start];
        let rest = &molecule[start..];
        for (pattern, replacement) in &replacements {
            if rest.starts_with(pattern) {
                let suffix = rest.replacen(pattern, replacement, 1);
                let new = format!("{prefix}{suffix}");
                seen.insert(new);
            }
        }
    }

    let res = seen.len();
    println!("{res}");

    Ok(())
}
