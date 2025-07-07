use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/13-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/13.txt")?;

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
        let mut min_cost = None;

        for a_count in 0..=100 {
            for b_count in 0..=100 {
                let x = a_count * a_x + b_count * b_x;
                let y = a_count * a_y + b_count * b_y;
                if x > prize_x || y > prize_y {
                    continue;
                }

                if x == prize_x && y == prize_y {
                    let cost = a_count * a_cost + b_count * b_cost;
                    if let Some(c) = min_cost {
                        if cost < c {
                            min_cost = Some(cost);
                        }
                    } else {
                        min_cost = Some(cost);
                    }
                }
            }
        }

        if let Some(cost) = min_cost {
            answer += cost;
        }
    }

    println!("{answer}");

    Ok(())
}
