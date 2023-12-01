use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/14.txt")?;
    let regex = Regex::new(
        r"^\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$",
    )?;

    let mut reindeer = input
        .lines()
        .map(|line| {
            let Some(caps) = regex.captures(line) else {
                unreachable!();
            };
            let (_, [speed, travel, rest]) = caps.extract();
            let speed: i32 = speed.parse()?;
            let travel: i32 = travel.parse()?;
            let rest: i32 = rest.parse()?;
            Ok((speed, travel, rest, true, 0, 0, 0))
        })
        .collect::<Result<Vec<_>>>()?;

    let duration = 2503;
    for _ in 0..duration {
        for (ref speed, ref travel, ref rest, traveling, distance, t, _) in reindeer.iter_mut() {
            *t += 1;
            if *traveling {
                *distance += *speed;
                if *t == *travel {
                    *traveling = false;
                    *t = 0;
                }
            } else {
                if *t == *rest {
                    *traveling = true;
                    *t = 0;
                }
            }
        }

        let Some(best) = reindeer.iter().map(|r| r.4).max() else {
            unreachable!()
        };

        for winner in reindeer.iter_mut().filter(|r| r.4 == best) {
            winner.6 += 1;
        }
    }

    let Some(res) = reindeer.into_iter().map(|r| r.6).max() else {
        unreachable!()
    };
    println!("{res}");

    Ok(())
}
