use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/24.txt")?;

    let mut ans: i64 = 0;

    let min = 200000000000000f64;
    let max = 400000000000000f64;

    let hail = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").unwrap();
            let (px, py, pz) = p
                .split(", ")
                .map(|p| p.trim().parse::<f64>().unwrap())
                .next_tuple()
                .unwrap();
            let (vx, vy, vz) = v
                .split(", ")
                .map(|p| p.trim().parse::<f64>().unwrap())
                .next_tuple()
                .unwrap();
            (px, py, pz, vx, vy, vz)
        })
        .collect_vec();

    for (i, a) in hail.iter().enumerate() {
        for (j, b) in hail.iter().enumerate() {
            if j <= i {
                continue;
            }

            let &(apx, apy, _, avx, avy, _) = a;
            let &(bpx, bpy, _, bvx, bvy, _) = b;

            let d = avx * bvy - avy * bvx;
            let u = (apy * bvx + bvy * bpx - bpy * bvx - bvy * apx) / d;
            let v = (apx + avx * u - bpx) / bvx;
            if u <= 0.0 && v <= 0.0 {
                continue;
            }

            let a2x = apx + avx;
            let a2y = apy + avy;
            let b2x = bpx + bvx;
            let b2y = bpy + bvy;

            let m1 = (a2y - apy) / (a2x - apx); // slope of line p1->n1
            let m2 = (b2y - bpy) / (b2x - bpx); // slope of line p2->n2

            let b1 = apy - m1 * apx; // y-intercept of line p1->n1
            let b2 = bpy - m2 * bpx; // y-intercept of line p2->n2

            let px = (b2 - b1) / (m1 - m2); // collision x
            let py = m1 * px + b1; // collision y

            if px < min || px > max || py < min || py > max {
                continue;
            }

            if avx < 0.0 {
                if px > apx {
                    continue;
                }
            } else if avx > 0.0 {
                if px < apx {
                    continue;
                }
            }

            if avy < 0.0 {
                if py > apy {
                    continue;
                }
            } else if avy > 0.0 {
                if py < apy {
                    continue;
                }
            }

            if bvx < 0.0 {
                if px > bpx {
                    continue;
                }
            } else if bvx > 0.0 {
                if px < bpx {
                    continue;
                }
            }

            if bvy < 0.0 {
                if py > bpy {
                    continue;
                }
            } else if bvy > 0.0 {
                if py < bpy {
                    continue;
                }
            }

            ans += 1;
        }
    }

    println!("{ans}");
    Ok(())
}
