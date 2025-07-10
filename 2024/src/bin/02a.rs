use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/02-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/02.txt")?;

    let mut safe_reports = 0;

    for line in input.lines() {
        let mut increasing = None;
        let mut safe = true;

        for (this, next) in line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .tuple_windows()
        {
            if let Some(increasing) = increasing {
                if increasing {
                    if next < this {
                        safe = false;
                    }
                } else if this < next {
                    safe = false;
                }
            } else {
                increasing = Some(this < next);
            }

            if (next - this).abs() > 3 {
                safe = false;
            }
            if next == this {
                safe = false;
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    println!("{safe_reports}");
    Ok(())
}
