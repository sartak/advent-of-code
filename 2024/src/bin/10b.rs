use anyhow::Result;
use itertools::Itertools;

fn rate(trail: &[Vec<u32>], x: usize, y: usize) -> usize {
    let mut rating = 0;
    let mut queue = Vec::new();
    queue.push((0, x as i32, y as i32));

    while let Some((height, x, y)) = queue.pop() {
        if height == 9 {
            rating += 1;
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = x + dx;
            let y = y + dy;

            if x < 0 || y < 0 {
                continue;
            }

            let Some(row) = trail.get(y as usize) else {
                continue;
            };

            let Some(&h) = row.get(x as usize) else {
                continue;
            };

            if h == height + 1 {
                queue.push((h, x, y));
            }
        }
    }

    rating
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/10-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/10.txt")?;

    let trail = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(99))
                .collect_vec()
        })
        .collect_vec();

    let mut answer = 0;
    for (y, row) in trail.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            if height == 0 {
                let s = rate(&trail, x, y);
                answer += s;
            }
        }
    }

    println!("{answer}");

    Ok(())
}
