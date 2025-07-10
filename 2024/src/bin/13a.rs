use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/13-example.txt"
    } else {
        "input/13.txt"
    })?;

    let mut games = Vec::new();

    {
        let mut a_x = None;
        let mut a_y = None;
        let mut b_x = None;
        let mut b_y = None;

        let a_rx = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
        let b_rx = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
        let prize_rx = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            if let Some(caps) = a_rx.captures(line) {
                let (_, [x, y]) = caps.extract();
                a_x = Some(x.parse::<i64>().unwrap());
                a_y = Some(y.parse::<i64>().unwrap());
                continue;
            }

            if let Some(caps) = b_rx.captures(line) {
                let (_, [x, y]) = caps.extract();
                b_x = Some(x.parse::<i64>().unwrap());
                b_y = Some(y.parse::<i64>().unwrap());
                continue;
            }

            if let Some(caps) = prize_rx.captures(line) {
                let (_, [x, y]) = caps.extract();
                let prize_x = x.parse::<i64>().unwrap();
                let prize_y = y.parse::<i64>().unwrap();

                games.push((
                    a_x.unwrap(),
                    a_y.unwrap(),
                    b_x.unwrap(),
                    b_y.unwrap(),
                    prize_x,
                    prize_y,
                ));
                continue;
            }

            panic!("Unhandled line {line}");
        }
    }

    let a_cost = 3;
    let b_cost = 1;
    let mut answer = 0;

    for (a_x, a_y, b_x, b_y, prize_x, prize_y) in games {
        // prize_x = a_x * a_count + b_x * b_count
        // 0 = a_x * a_count + b_x * b_count - prize_x
        // -a_x * a_count = b_x * b_count - prize_x
        // a_count = (b_x * b_count - prize_x) / -a_x

        // prize_y = a_y * a_count + b_y * b_count
        // 0 = a_y * a_count + b_y * b_count - prize_y
        // -b_y * b_count = a_y * a_count - prize_y
        // b_count = (a_y * a_count - prize_y) / -b_y

        // a_count = (b_x * ((a_y * a_count - prize_y) / -b_y) - prize_x) / -a_x
        // a_count = (b_x * ((a_y * a_count - prize_y) / -b_y) / -a_x) + prize_x/a_x
        // a_count = (-b_x/a_x * ((a_y * a_count - prize_y) / -b_y)) + prize_x/a_x
        // a_count = (-b_x/a_x * (-a_y/b_y * a_count + prize_y/b_y)) + prize_x/a_x
        // a_count = b_x/a_x * a_y/b_y * a_count - b_x/a_x * prize_y/b_y + prize_x/a_x
        // -b_x/a_x * a_y/b_y * a_count + a_count = -b_x/a_x * prize_y/b_y + prize_x/a_x
        // (-b_x/a_x * a_y/b_y + 1) * a_count = -b_x/a_x * prize_y/b_y + prize_x/a_x
        // a_count = (-b_x/a_x * prize_y/b_y + prize_x/a_x) / (-b_x/a_x * a_y/b_y + 1)
        // b_count = (a_y * ((-b_x/a_x * prize_y/b_y + prize_x/a_x) / (-b_x/a_x * a_y/b_y + 1)) - prize_y) / -b_y

        let (a_count, b_count) = {
            let a_x = a_x as f64;
            let a_y = a_y as f64;
            let b_x = b_x as f64;
            let b_y = b_y as f64;
            let prize_x = prize_x as f64;
            let prize_y = prize_y as f64;

            let a_count =
                (-b_x / a_x * prize_y / b_y + prize_x / a_x) / (-b_x / a_x * a_y / b_y + 1.0);
            let b_count = (a_y
                * ((-b_x / a_x * prize_y / b_y + prize_x / a_x) / (-b_x / a_x * a_y / b_y + 1.0))
                - prize_y)
                / -b_y;
            (a_count, b_count)
        };

        let a_count = a_count.round() as i64;
        let b_count = b_count.round() as i64;

        if a_x * a_count + b_x * b_count != prize_x {
            continue;
        }
        if a_y * a_count + b_y * b_count != prize_y {
            continue;
        }

        answer += a_cost * a_count + b_cost * b_count;
    }

    println!("{answer}");

    Ok(())
}
