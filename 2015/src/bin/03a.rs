use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/03.txt")?;
    let mut houses = std::collections::HashSet::new();
    let (mut x, mut y) = (0, 0);

    let mut deliver = |x, y| {
        houses.insert((x, y));
    };

    deliver(x, y);

    for c in input.chars() {
        let (dx, dy) = match c {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => continue,
        };

        x += dx;
        y += dy;
        deliver(x, y);
    }

    let res = houses.len();
    println!("{res}");

    Ok(())
}
