use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/02.txt")?;
    let mut code = Vec::new();

    let digits = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];

    let mut x: i32 = 1;
    let mut y: i32 = 1;

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

            y = ny;
            x = nx;
        }

        code.push(digits[y as usize][x as usize]);
    }

    println!("{}", code.join(""));
    Ok(())
}
