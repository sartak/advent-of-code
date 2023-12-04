use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/04.txt")?;
    let input = input.lines().collect_vec();
    let mut cards = (0..input.len()).collect_vec();
    let mut s = 0;
    while let Some(n) = cards.pop() {
        s += 1;
        let (_, nums) = input[n].split_once(": ").unwrap();
        let (winners, have) = nums.split_once(" | ").unwrap();
        let winners = winners
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();
        let have = have
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();

        let mut w = 0;
        for n in have {
            if winners.contains(&n) {
                w += 1;
            }
        }

        for i in (n + 1)..=(n + w) {
            if i < input.len() {
                cards.push(i);
            }
        }
    }

    println!("{s}");
    Ok(())
}
