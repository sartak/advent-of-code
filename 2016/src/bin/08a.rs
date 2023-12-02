use std::collections::VecDeque;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
}
use Axis::*;

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    Rotate {
        axis: Axis,
        coord: usize,
        amount: usize,
    },
}
use Instruction::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/08.txt")?;
    const WIDTH: usize = 50;
    const HEIGHT: usize = 6;
    let mut display = [[false; WIDTH]; HEIGHT];

    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let instruction = words.next().unwrap();

            match instruction {
                "rect" => {
                    let dimensions = words.next().unwrap();
                    let (w, h) = dimensions.split_once('x').unwrap();
                    let w = w.parse::<usize>().unwrap();
                    let h = h.parse::<usize>().unwrap();
                    Rect(w, h)
                }
                "rotate" => {
                    let _ = words.next().unwrap();
                    let coord = words.next().unwrap();
                    let (axis, coord) = coord.split_once('=').unwrap();
                    let axis = if axis == "x" { X } else { Y };
                    let coord = coord.parse::<usize>().unwrap();
                    let _ = words.next();
                    let amount = words.next().unwrap().parse::<usize>().unwrap();
                    Rotate {
                        axis,
                        coord,
                        amount,
                    }
                }
                _ => unreachable!(),
            }
        })
        .for_each(|instruction| {
            match instruction {
                Rect(w, h) => {
                    for y in 0..h {
                        for x in 0..w {
                            display[y][x] = true;
                        }
                    }
                }
                Rotate {
                    axis,
                    coord,
                    amount,
                } => match axis {
                    X => {
                        let mut v = VecDeque::with_capacity(HEIGHT);
                        for y in 0..HEIGHT {
                            v.push_back(display[(HEIGHT + y - amount) % HEIGHT][coord]);
                        }
                        for (y, n) in v.into_iter().enumerate() {
                            display[y][coord] = n;
                        }
                    }
                    Y => {
                        let mut v = VecDeque::with_capacity(WIDTH);
                        for x in 0..WIDTH {
                            v.push_back(display[coord][(WIDTH + x - amount) % WIDTH]);
                        }
                        for (x, n) in v.into_iter().enumerate() {
                            display[coord][x] = n;
                        }
                    }
                },
            };
        });

    let count: usize = display
        .into_iter()
        .map(|r| r.into_iter().filter(|&c| c).count())
        .sum();
    println!("{count}");

    Ok(())
}
