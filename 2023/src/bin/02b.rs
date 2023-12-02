use anyhow::Result;
use std::cmp::max;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/02.txt")?;

    let mut sum = 0;
    for line in input.lines() {
        let (_, gems) = line.split_once(": ").unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let hands = gems.split("; ");
        for hand in hands {
            let gems = hand.split(", ");
            for gem in gems {
                let (count, color) = gem.split_once(' ').unwrap();
                let count: i64 = count.parse().unwrap();
                match color {
                    "red" => red = max(red, count),
                    "green" => green = max(green, count),
                    "blue" => blue = max(blue, count),
                    _ => unreachable!(),
                };
            }
        }
        let power = red * green * blue;
        sum += power;
    }

    println!("{sum}");
    Ok(())
}
