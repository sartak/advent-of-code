use anyhow::Result;
use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;

#[memoize]
fn arrange(diagram: Vec<String>, lengths: Vec<usize>) -> usize {
    if lengths.is_empty() {
        if diagram.iter().any(|d| d.contains('#')) {
            return 0;
        }
        return 1;
    }
    if diagram.is_empty() {
        return 0;
    }

    let (d, dr) = diagram.split_at(1);
    let (l, lr) = lengths.split_at(1);
    let d = &d[0];
    let l = l[0];

    if l > d.len() {
        if d.contains('#') {
            return 0;
        }
        return arrange(dr.to_owned(), lengths);
    }
    if d.len() == l {
        let mut res = arrange(dr.to_owned(), lr.to_owned());
        if !d.contains('#') {
            res += arrange(dr.to_owned(), lengths);
        }
        return res;
    }

    let mut ds = d
        .chars()
        .map(|c| match c {
            '?' => false,
            '#' => true,
            _ => panic!(),
        })
        .dedup_with_count();

    let Some(front) = ds.next() else { panic!() };

    let (count, on) = front;

    if on {
        if count > l {
            0
        } else if count == l {
            if d.len() == count {
                arrange(dr.to_owned(), lr.to_owned())
            } else {
                // consume a ?
                let mut rest = vec![d.split_at(count + 1).1.to_owned()];
                if rest[0].is_empty() {
                    arrange(dr.to_owned(), lr.to_owned())
                } else {
                    rest.extend(dr.to_owned());
                    arrange(rest, lr.to_owned())
                }
            }
        } else {
            // consume count from the iterator
            let mut used = count;
            while used < l {
                let Some((nc, on)) = ds.next() else {
                    return 0;
                };
                used += nc;
                if used > nc {
                    if on {
                        return 0;
                    } else {
                        used = l
                    }
                }
            }

            let r = d.split_at(used).1;
            if r.is_empty() {
                arrange(dr.to_owned(), lr.to_owned())
            } else if r.starts_with('?') {
                let mut rest = vec![d.split_at(used + 1).1.to_owned()];
                rest.extend(dr.to_owned());
                arrange(rest, lr.to_owned())
            } else {
                0
            }
        }
    } else {
        let on = {
            let nl = l - 1;
            if nl == 0 {
                if count - 1 > 0 {
                    // 2 because this + separator
                    let mut rest = vec![String::from(d.split_at(2).1)];
                    if rest[0].is_empty() {
                        arrange(dr.to_owned(), lr.to_owned())
                    } else {
                        rest.extend(dr.to_owned());
                        arrange(rest, lr.to_owned())
                    }
                } else {
                    match ds.next() {
                        // we consumed this whole chunk
                        None => arrange(dr.to_owned(), lr.to_owned()),
                        // we need a separator but we saw #
                        Some((_, true)) => 0,
                        // we shouldn't see two ? chunks in a row
                        Some((_, false)) => unreachable!(),
                    }
                }
            } else {
                let r = format!("#{}", d.split_at(1).1.to_owned());
                let mut rest = vec![r];
                rest.extend(dr.to_owned());
                arrange(rest, lengths.clone())
            }
        };

        let off = {
            let mut rest = vec![d.split_at(1).1.to_owned()];
            rest.extend(dr.to_owned());
            arrange(rest, lengths)
        };
        on + off
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/12.txt")?;
    let lines = input.lines().collect_vec();

    let count: usize = lines
        .into_par_iter()
        .map(|line| {
            let (diagram, lengths) = line.split_once(' ').unwrap();

            let diagram = [diagram; 5].join("?");
            let lengths = [lengths; 5].join(",");

            let lengths = lengths
                .split(',')
                .map(|l| l.parse::<usize>().unwrap())
                .collect_vec();

            let diagram = diagram
                .split('.')
                .filter(|n| !n.is_empty())
                .map(String::from)
                .collect_vec();

            arrange(diagram, lengths)
        })
        .sum();

    println!("{count}");
    Ok(())
}
