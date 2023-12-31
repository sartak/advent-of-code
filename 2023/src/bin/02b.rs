use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/02.txt")?;

    let sum: i64 = input
        .lines()
        .map(|line| {
            let (_, rounds) = line.split_once(": ").unwrap();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for round in rounds.split("; ") {
                for gem in round.split(", ") {
                    let (count, color) = gem.split_once(' ').unwrap();
                    let count = count.parse().unwrap();
                    match color {
                        "red" => red = red.max(count),
                        "green" => green = green.max(count),
                        "blue" => blue = blue.max(count),
                        _ => unreachable!(),
                    };
                }
            }

            red * green * blue
        })
        .sum();

    println!("{sum}");

    Ok(())
}
