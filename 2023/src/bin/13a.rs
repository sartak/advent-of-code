use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/13.txt")?;
    let mut maps = vec![];
    let mut map = vec![];
    for line in input.lines() {
        if line.is_empty() {
            if !map.is_empty() {
                maps.push(map);
                map = vec![];
            }
        } else {
            map.push(line.chars().collect_vec());
        }
    }
    maps.push(map);

    let mut ans = 0;
    'map: for map in maps.iter() {
        let yy = map.len();
        let xx = map[0].len();

        'mx: for mx in 0..(xx - 1) {
            for x in 0..=mx {
                let rx = 2 * mx - x + 1;
                if rx < xx {
                    for row in map {
                        if row[x] != row[rx] {
                            continue 'mx;
                        }
                    }
                }
            }

            ans += mx + 1;
            continue 'map;
        }

        'my: for my in 0..(yy - 1) {
            for y in 0..=my {
                let ry = 2 * my - y + 1;
                if ry < yy {
                    for x in 0..map[0].len() {
                        if map[y][x] != map[ry][x] {
                            continue 'my;
                        }
                    }
                }
            }

            ans += 100 * (my + 1);
            continue 'map;
        }
    }

    println!("{ans}");

    Ok(())
}
