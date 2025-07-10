use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/25-example.txt"
    } else {
        "input/25.txt"
    })?;

    let mut blocks = Vec::new();
    let mut block = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            blocks.push(block);
            block = Vec::new();
        } else {
            block.push(line.chars().collect_vec());
        }
    }

    let height = block.len() - 2;

    blocks.push(block);

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for block in blocks {
        let parsed = (0..block[0].len())
            .map(|x| {
                (0..block.len())
                    .map(|y| if block[y][x] == '#' { 1 } else { 0 })
                    .sum::<usize>()
                    - 1
            })
            .collect_vec();

        if block[0] == vec!['#', '#', '#', '#', '#'] {
            locks.push(parsed);
        } else if *block.last().unwrap() == vec!['#', '#', '#', '#', '#'] {
            keys.push(parsed);
        } else {
            panic!("Unhandled block {block:?}");
        }
    }

    let mut answer = 0;
    for key in &keys {
        'lock: for lock in &locks {
            for (&kc, &lc) in key.iter().zip(lock) {
                if kc + lc > height {
                    continue 'lock;
                }
            }
            answer += 1;
        }
    }

    println!("{answer}");

    Ok(())
}
