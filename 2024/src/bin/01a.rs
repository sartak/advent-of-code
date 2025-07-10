use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/01-example.txt"
    } else {
        "input/01.txt"
    })?;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let l = words.next().unwrap();
        let r = words.next().unwrap();
        assert!(words.next().is_none());

        left.push(l.parse::<i32>()?);
        right.push(r.parse::<i32>()?);
    }

    left.sort();
    right.sort();

    let mut dist = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        let d = (l - r).abs();
        dist += d;
    }

    println!("{dist}");

    Ok(())
}
