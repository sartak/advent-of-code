use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/02.txt")?;

    let sum: i64 = input
        .lines()
        .filter_map(|line| {
            let (game, rounds) = line.split_once(": ").unwrap();
            let (_, game) = game.split_once(' ').unwrap();
            let id: i64 = game.parse().unwrap();

            for round in rounds.split("; ") {
                for gem in round.split(", ") {
                    let (count, color) = gem.split_once(' ').unwrap();
                    let limit = match color {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => unreachable!(),
                    };

                    let count: i64 = count.parse().unwrap();
                    if count > limit {
                        return None;
                    }
                }
            }

            Some(id)
        })
        .sum();

    println!("{sum}");

    Ok(())
}
