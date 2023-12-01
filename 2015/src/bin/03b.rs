use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/03.txt")?;
    let mut houses = std::collections::HashSet::new();
    let (mut sx, mut sy) = (0, 0);
    let (mut rx, mut ry) = (0, 0);

    let mut deliver = |x, y| {
        houses.insert((x, y));
    };

    deliver(sx, sy);
    deliver(rx, ry);

    for (i, c) in input.chars().enumerate() {
        let (dx, dy) = match c {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => continue,
        };

        if i % 2 == 0 {
            sx += dx;
            sy += dy;
            deliver(sx, sy);
        } else {
            rx += dx;
            ry += dy;
            deliver(rx, ry);
        }
    }

    let res = houses.len();
    println!("{res}");

    Ok(())
}
