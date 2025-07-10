use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/02-example.txt"
    } else {
        "input/02.txt"
    })?;

    let mut safe_reports = 0;

    'line: for line in input.lines() {
        let levels = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec();

        // -1 won't match any index, so it tries without skipping
        for skip in -1..(levels.len() as i32) {
            let mut increasing = None;
            let mut safe = true;

            for (this, next) in levels
                .iter()
                .enumerate()
                .filter_map(|(i, level)| if i as i32 == skip { None } else { Some(level) })
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
                continue 'line;
            }
        }
    }

    println!("{safe_reports}");
    Ok(())
}
