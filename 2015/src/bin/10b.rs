use anyhow::Result;
use itertools::Itertools;

fn speak(input: &str) -> String {
    input
        .chars()
        .peekable()
        .batching(|it| match it.next() {
            None => None,
            Some(c) => {
                let mut count = 1;
                loop {
                    let Some(n) = it.peek() else { break };
                    if *n != c {
                        break;
                    }
                    it.next();
                    count += 1;
                }
                Some(format!("{count}{c}"))
            }
        })
        .collect()
}

fn main() -> Result<()> {
    let seq = (1..=50).fold(String::from("1321131112"), |seq, _| speak(&seq));
    let res = seq.len();
    println!("{res}");

    Ok(())
}
