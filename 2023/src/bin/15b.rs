use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/15.txt")?;

    let mut boxes = vec![vec![]; 256];

    input.lines().for_each(|line| {
        line.split(',').for_each(|word| {
            let (label, focal) = if let Some(label) = word.strip_suffix('-') {
                (label, None)
            } else {
                let (label, focal) = word.split_once('=').unwrap();
                let focal = focal.parse::<usize>().unwrap();
                (label, Some(focal))
            };

            let label = String::from(label);

            let mut hash = 0;
            label.chars().for_each(|c| {
                if c == '\n' {
                    return;
                }

                hash += c as i64;
                hash *= 17;
                hash %= 256;
            });

            let b = boxes.get_mut(hash as usize).unwrap();

            if let Some(focal) = focal {
                if let Some(i) = b.iter().position(|(l, _)| *l == label) {
                    b[i].1 = focal;
                } else {
                    b.push((label, focal));
                }
            } else {
                b.retain(|(l, _)| *l != label);
            }
        });
    });

    let ans = boxes
        .into_iter()
        .enumerate()
        .map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(|(j, (_, focal))| (i + 1) * (j + 1) * focal)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{ans}");
    Ok(())
}
