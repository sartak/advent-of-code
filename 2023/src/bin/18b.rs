use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/18.txt")?;
    let lines = input.lines().collect_vec();
    let rx = Regex::new(r"^\w \d+ \(\#(\w+)(\w)\)$")?;

    let mut x = 0;
    let mut y = 0;

    let mut coords = vec![(x, y)];

    lines.iter().for_each(|&line| {
        let caps = rx.captures(line).unwrap();
        let steps = i64::from_str_radix(caps.get(1).unwrap().as_str(), 16).unwrap();
        let dir = caps.get(2).unwrap().as_str();

        let dir = match dir {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!(),
        };

        let mut dx = 0;
        let mut dy = 0;
        match dir {
            "R" => dx = steps,
            "L" => dx = -steps,
            "U" => dy = -steps,
            "D" => dy = steps,
            _ => panic!(),
        };

        x += dx;
        y += dy;

        coords.push((x, y));
    });

    let mut ans = 0;
    for i in 0..coords.len() - 1 {
        let (xi, yi) = coords[i];
        let (x1, y1) = coords[i + 1];

        // shoelace theorem
        ans += (y1 + yi) * (xi - x1);

        // count perimeter too
        ans += (y1 - yi).abs() + (x1 - xi).abs();
    }
    ans >>= 1;

    // no idea why i have to add one but it worked for example and input
    ans += 1;

    println!("{ans}");
    Ok(())
}
