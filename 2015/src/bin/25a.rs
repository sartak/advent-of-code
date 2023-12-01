use anyhow::Result;

fn main() -> Result<()> {
    let mut code: i64 = 20151125;
    let row = 2947;
    let col = 3029;
    let mut x = 1;
    let mut y = 1;

    let code = loop {
        if x == col && y == row {
            break code;
        }

        code = (code * 252533) % 33554393;

        if y == 1 {
            y = x + 1;
            x = 1;
        } else {
            y -= 1;
            x += 1;
        }
    };

    println!("{code}");

    Ok(())
}
