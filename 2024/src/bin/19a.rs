use anyhow::Result;
use itertools::Itertools;

fn possible(towels: &[&str], desired: &str) -> bool {
    let mut queue = Vec::new();
    queue.push(desired);

    while let Some(d) = queue.pop() {
        if d.is_empty() {
            return true;
        }

        for &towel in towels {
            if let Some(r) = d.strip_prefix(towel) {
                queue.push(r);
            }
        }
    }

    false
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/19-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/19.txt")?;

    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").collect_vec();
    assert!(lines.next().unwrap().is_empty());

    let mut answer = 0;
    for desired in lines {
        if possible(&towels, desired) {
            answer += 1;
        }
    }

    println!("{answer}");

    Ok(())
}
