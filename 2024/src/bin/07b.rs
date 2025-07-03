use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/07-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/07.txt")?;

    let mut answer = 0;

    for line in input.lines() {
        let (output, inputs) = line.split_once(':').unwrap();
        let output = output.parse::<i64>().unwrap();
        let inputs = inputs
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        let mut queue = VecDeque::new();
        queue.push_back((inputs[0], 1));

        while let Some((result, index)) = queue.pop_front() {
            if let Some(v) = inputs.get(index) {
                queue.push_back((result + v, index + 1));
                queue.push_back((result * v, index + 1));

                let concat = format!("{result}{v}");
                let concat = concat.parse().unwrap();
                queue.push_back((concat, index + 1));
            } else if result == output {
                answer += output;
                break;
            }
        }
    }

    println!("{answer}");

    Ok(())
}
