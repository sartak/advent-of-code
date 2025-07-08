use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/22-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/22.txt")?;

    let mut answer = 0;

    for line in input.lines() {
        let mut n: u64 = line.parse().unwrap();

        for _ in 0..2000 {
            let a = n * 64;
            n ^= a;
            n %= 16777216;

            let a = n / 32;
            n ^= a;
            n %= 16777216;

            let a = n * 2048;
            n ^= a;
            n %= 16777216;
        }

        answer += n;
    }

    println!("{answer}");

    Ok(())
}
