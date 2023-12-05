use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/09.txt")?;
    let regex = Regex::new(r"^\((\d+)x(\d+)\)")?;

    let len: usize = input
        .lines()
        .map(|line| {
            let mut rest = line;
            let mut decompressed = 0;
            while !rest.is_empty() {
                if let Some(caps) = regex.captures(rest) {
                    let size = caps.get(0).unwrap().as_str().len();
                    let subsequent: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                    let repetitions: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                    decompressed += subsequent * repetitions;
                    let skip = size + subsequent;
                    rest = &rest[skip..];
                } else {
                    rest = &rest[1..];
                    decompressed += 1;
                }
            }
            decompressed
        })
        .sum();

    println!("{len}");

    Ok(())
}
