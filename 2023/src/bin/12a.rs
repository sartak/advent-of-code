use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/12.txt")?;

    let count: usize = input
        .lines()
        .map(|line| {
            let (diagram, lengths) = line.split_once(' ').unwrap();
            let lengths = lengths
                .split(',')
                .map(|l| l.parse::<usize>().unwrap())
                .collect_vec();
            let re = lengths.into_iter().map(|len| "#".repeat(len)).join(r"\.+");
            let re = format!(r"^\.*{re}\.*$");
            let re = Regex::new(&re).unwrap();

            let diagram = diagram
                .chars()
                .map(|c| match c {
                    '?' => None,
                    '.' => Some(true),
                    '#' => Some(false),
                    _ => panic!(),
                })
                .collect_vec();
            let size = diagram.iter().filter(|s| s.is_none()).count();

            (0..(1 << size))
                .filter(|i| {
                    let mut j = 0;
                    let diagram = diagram
                        .iter()
                        .map(|s| match s {
                            None => {
                                let off = (i & (1 << j)) == 0;
                                j += 1;
                                off
                            }
                            Some(t) => *t,
                        })
                        .map(|b| if b { '.' } else { '#' })
                        .collect::<String>();
                    re.is_match(&diagram)
                })
                .count()
        })
        .sum();

    println!("{count}");
    Ok(())
}
