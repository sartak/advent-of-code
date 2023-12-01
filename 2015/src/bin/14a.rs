use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/14.txt")?;
    let regex = Regex::new(
        r"^\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$",
    )?;
    let duration = 2503;

    let res = input
        .lines()
        .map(|line| {
            let Some(caps) = regex.captures(line) else {
                unreachable!();
            };
            let (_, [speed, travel, rest]) = caps.extract();
            let speed: i32 = speed.parse()?;
            let travel: i32 = travel.parse()?;
            let rest: i32 = rest.parse()?;

            let mut t = duration;
            let mut d = 0;
            while t > 0 {
                if t < travel {
                    d += speed * t;
                    break;
                } else {
                    d += speed * travel;
                    t -= travel;
                }

                t -= rest;
            }

            Ok(d)
        })
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .max();

    let Some(res) = res else { unreachable!() };

    println!("{res}");

    Ok(())
}
