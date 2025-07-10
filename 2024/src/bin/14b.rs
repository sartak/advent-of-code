use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/14-example.txt"
    } else {
        "input/14.txt"
    })?;

    let (width, height) = if cfg!(debug_assertions) {
        (11, 7)
    } else {
        (101, 103)
    };

    let meridian = width >> 1;
    let equator = height >> 1;

    let rx = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();
    let mut robots = input
        .lines()
        .map(|line| {
            let (_, [px, py, vx, vy]) = rx.captures(line).unwrap().extract();
            Robot {
                px: px.parse().unwrap(),
                py: py.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect_vec();

    for tick in 1.. {
        for robot in robots.iter_mut() {
            robot.px = (robot.px + robot.vx).rem_euclid(width);
            robot.py = (robot.py + robot.vy).rem_euclid(height);
        }

        println!("{tick}");
        for y in 0..height {
            for x in 0..width {
                let mut count = 0;
                for robot in &robots {
                    if x == robot.px && y == robot.py {
                        count += 1;
                    }
                }
                if count == 0 {
                    print!(".");
                } else {
                    print!("{count}");
                }
            }
            println!();
        }
        println!();

        if (tick - 35) % 101 == 0 {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;
        }
    }

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    for robot in robots {
        let is_left = if robot.px == meridian {
            continue;
        } else {
            robot.px < meridian
        };

        let is_top = if robot.py == equator {
            continue;
        } else {
            robot.py < equator
        };

        match (is_top, is_left) {
            (true, true) => tl += 1,
            (false, true) => bl += 1,
            (true, false) => tr += 1,
            (false, false) => br += 1,
        }
    }

    println!("{}", tl * bl * tr * br);

    Ok(())
}
