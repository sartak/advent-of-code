use anyhow::Result;
use itertools::Itertools;

struct Brick {
    ax: usize,
    ay: usize,
    az: usize,
    bx: usize,
    by: usize,
    bz: usize,
}

fn blocks(t: &Brick, b: &Brick) -> bool {
    if t.az != b.bz + 1 {
        return false;
    }

    if t.bx < b.ax || t.ax > b.bx {
        return false;
    }

    if t.by < b.ay || t.ay > b.by {
        return false;
    }

    true
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/22.txt")?;

    let mut bricks = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('~').unwrap();
            let (ax, ay, az) = a
                .split(',')
                .map(|p| p.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap();
            let (bx, by, bz) = b
                .split(',')
                .map(|p| p.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap();

            assert!(ax <= bx);
            assert!(ay <= by);
            assert!(az <= bz);

            Brick {
                ax,
                ay,
                az,
                bx,
                by,
                bz,
            }
        })
        .collect_vec();

    loop {
        let mut moved = false;
        'm: for m in 0..bricks.len() {
            if bricks[m].az == 1 {
                continue;
            }

            for b in 0..bricks.len() {
                if m == b {
                    continue;
                }

                if blocks(&bricks[m], &bricks[b]) {
                    continue 'm;
                }
            }

            bricks[m].az -= 1;
            bricks[m].bz -= 1;
            moved = true;
        }

        if !moved {
            break;
        }
    }

    let mut ans = 0;
    'd: for (d, disint) in bricks.iter().enumerate() {
        't: for (t, top) in bricks.iter().enumerate() {
            if t == d {
                continue;
            }

            if !blocks(top, disint) {
                continue;
            }

            for (b, bot) in bricks.iter().enumerate() {
                if b == d || b == t {
                    continue;
                }

                if blocks(top, bot) {
                    continue 't;
                }
            }

            // top brick would fall, so we need to keep disint
            continue 'd;
        }

        ans += 1;
    }

    println!("{ans}");
    Ok(())
}
