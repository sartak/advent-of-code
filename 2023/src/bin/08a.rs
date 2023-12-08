use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/08.txt")?;
    let mut lines = input.lines();
    let rx = Regex::new(r"(\w+) = \((\w+), (\w+)\)")?;

    let dirs = lines.next().unwrap();

    let _empty = lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let caps = rx.captures(line).unwrap();
        let start = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();

        map.insert(start, (left, right));
    }

    let mut spot = "AAA";
    for (i, dir) in dirs.chars().cycle().enumerate() {
        let options = map.get(spot).unwrap();
        let next = match dir {
            'L' => options.0,
            'R' => options.1,
            _ => panic!(),
        };
        spot = next;
        if spot == "ZZZ" {
            println!("{}", i + 1);
            break;
        }
    }

    Ok(())
}
