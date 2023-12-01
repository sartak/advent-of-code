use anyhow::Result;
use std::ops::ControlFlow;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/01.txt")?;
    let res = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .enumerate()
        .try_fold(0, |level, (pos, delta)| {
            let level = level + delta;
            if level == -1 {
                ControlFlow::Break(pos)
            } else {
                ControlFlow::Continue(level)
            }
        });

    let ControlFlow::Break(pos) = res else {
        unreachable!()
    };

    println!("{}", pos + 1);

    Ok(())
}
