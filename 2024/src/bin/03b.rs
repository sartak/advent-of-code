use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/03-example.txt"
    } else {
        "input/03.txt"
    })?;

    let rx = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for caps in rx.captures_iter(&input) {
        if caps.get(1).is_some() {
            if enabled {
                let x = caps[1].parse::<usize>().unwrap();
                let y = caps[2].parse::<usize>().unwrap();
                sum += x * y;
            }
        } else if &caps[0] == "do()" {
            enabled = true;
        } else if &caps[0] == "don't()" {
            enabled = false;
        } else {
            unreachable!("{:?}", caps.get(0));
        }
    }

    println!("{sum}");

    Ok(())
}
