use anyhow::Result;

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}
use Instruction::*;

fn main() -> Result<()> {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;
    let mut lights = [false; WIDTH * HEIGHT];
    let input = std::fs::read_to_string("input/06.txt")?;
    let regex = regex::Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")?;
    for line in input.lines() {
        let Some(caps) = regex.captures(line) else {
            unreachable!()
        };
        let (_, [instruction, x0, y0, x1, y1]) = caps.extract();
        let x0 = x0.parse::<usize>()?;
        let y0 = y0.parse::<usize>()?;
        let x1 = x1.parse::<usize>()?;
        let y1 = y1.parse::<usize>()?;

        let instruction = match instruction {
            "turn off" => TurnOff,
            "turn on" => TurnOn,
            "toggle" => Toggle,
            _ => unreachable!(),
        };

        for x in x0..=x1 {
            for y in y0..=y1 {
                let i = x + WIDTH * y;
                match instruction {
                    TurnOn => {
                        lights[i] = true;
                    }
                    TurnOff => {
                        lights[i] = false;
                    }
                    Toggle => {
                        lights[i] = !lights[i];
                    }
                }
            }
        }
    }

    let res = lights.iter().filter(|l| **l).count();
    println!("{res}");
    Ok(())
}
