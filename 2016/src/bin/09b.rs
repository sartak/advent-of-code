use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
enum Cell {
    Rep(usize, usize, usize),
    Char,
}
use Cell::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/09.txt")?;
    let regex = Regex::new(r"^\((\d+)x(\d+)\)")?;

    let len: usize = input
        .lines()
        .map(|line| {
            let mut rest = line;
            let mut cells = Vec::new();
            while !rest.is_empty() {
                if let Some(caps) = regex.captures(rest) {
                    let size = caps.get(0).unwrap().as_str().len();
                    let subsequent: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                    let repetitions: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                    rest = &rest[size..];
                    cells.push(Rep(size, subsequent, repetitions));
                } else {
                    rest = &rest[1..];
                    cells.push(Char);
                }
            }

            let mut decompressed = vec![0; cells.len()];
            for (i, cell) in cells.iter().enumerate().rev() {
                match *cell {
                    Rep(_, subsequent, repetitions) => {
                        let mut j = i;
                        let mut cursor = 0;
                        while cursor < subsequent {
                            j += 1;
                            decompressed[j] *= repetitions;
                            cursor += match cells[j] {
                                Rep(len, _, _) => len,
                                Char => 1,
                            };
                        }
                    }
                    Char => {
                        decompressed[i] = 1;
                    }
                }
            }

            decompressed.into_iter().sum::<usize>()
        })
        .sum();

    println!("{len}");

    Ok(())
}
