use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/07-example.txt"
    } else {
        "input/07.txt"
    })?;

    let mut answer = 0;

    'line: for line in input.lines() {
        let (output, inputs) = line.split_once(':').unwrap();
        let output = output.parse::<i64>().unwrap();
        let inputs = inputs
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();

        for bits in 0..(1 << (inputs.len() - 1)) {
            let mut result = inputs[0];

            for (i, val) in inputs.iter().enumerate().skip(1) {
                if bits & (1 << (i - 1)) > 0 {
                    result += val;
                } else {
                    result *= val;
                }
            }

            if result == output {
                answer += output;
                continue 'line;
            }
        }
    }

    println!("{answer}");

    Ok(())
}
