use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/01.txt")?;
    let res: i32 = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();

    println!("{res}");
    Ok(())
}
