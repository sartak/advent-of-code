use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/04.txt")?;
    let mut s: i64 = 0;

    for line in input.lines() {
        let (_, nums) = line.split_once(": ").unwrap();
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

        if w > 0 {
            s += (2i64).pow(w - 1);
        }
    }

    println!("{s}");
    Ok(())
}
