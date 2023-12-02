use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/02.txt")?;
    let mut games = vec![];

    let red = 12;
    let green = 13;
    let blue = 14;

    for line in input.lines() {
        let (game, gems) = line.split_once(": ").unwrap();
        let (_, game) = game.split_once(' ').unwrap();
        let game: i64 = game.parse().unwrap();

        let mut pos = true;
        let hands = gems.split("; ");
        for hand in hands {
            let gems = hand.split(", ");
            for gem in gems {
                let (count, color) = gem.split_once(' ').unwrap();
                let limit = match color {
                    "red" => red,
                    "green" => green,
                    "blue" => blue,
                    _ => unreachable!(),
                };

                let count: i64 = count.parse().unwrap();
                if count > limit {
                    pos = false;
                }
            }
        }

        if pos {
            games.push(game);
        }
    }

    let res: i64 = games.into_iter().sum();

    println!("{res}");
    Ok(())
}
