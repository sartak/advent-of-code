use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/02.txt")?;
    let mut code = Vec::new();

    let digits = [
        [None, None, Some("1"), None, None],
        [None, Some("2"), Some("3"), Some("4"), None],
        [Some("5"), Some("6"), Some("7"), Some("8"), Some("9")],
        [None, Some("A"), Some("B"), Some("C"), None],
        [None, None, Some("D"), None, None],
    ];

    let mut x: i32 = 0;
    let mut y: i32 = 2;

    for line in input.lines() {
        for dir in line.chars() {
            let (nx, ny) = match dir {
                'U' => (x, y - 1),
                'D' => (x, y + 1),
                'R' => (x + 1, y),
                'L' => (x - 1, y),
                _ => unreachable!(),
            };

            if ny < 0 || nx < 0 || ny as usize >= digits.len() || nx as usize >= digits[0].len() {
                continue;
            }

            if digits[ny as usize][nx as usize].is_none() {
                continue;
            }

            y = ny;
            x = nx;
        }

        code.push(digits[y as usize][x as usize].unwrap());
    }

    println!("{}", code.join(""));
    Ok(())
}
