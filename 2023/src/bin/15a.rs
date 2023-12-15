use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/15.txt")?;

    let mut ans: i64 = 0;

    input.split(',').for_each(|word| {
        let mut hash = 0;
        word.chars().for_each(|c| {
            if c == '\n' {
                return;
            }

            hash += c as i64;
            hash *= 17;
            hash %= 256;
        });
        ans += hash;
    });

    println!("{ans}");
    Ok(())
}
