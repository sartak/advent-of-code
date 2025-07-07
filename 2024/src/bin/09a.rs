use anyhow::Result;

#[derive(Debug)]
enum Block {
    Used(u32),
    Free,
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/09-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/09.txt")?;

    let mut blocks = Vec::new();
    let mut is_free = false;
    let mut id = 0;
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        let size = c.to_digit(10).unwrap();

        for _ in 0..size {
            blocks.push(if is_free {
                Block::Free
            } else {
                Block::Used(id)
            });
        }

        is_free = !is_free;
        if !is_free {
            id += 1;
        }
    }

    let mut first = 0;
    let mut last = blocks.len() - 1;

    'defrag: while first < last {
        while matches!(blocks[first], Block::Used(_)) {
            first += 1;
        }
        while matches!(blocks[last], Block::Free) {
            let Some(new) = last.checked_sub(1) else {
                break 'defrag;
            };
            last = new;
        }

        if first > last || first >= blocks.len() {
            break;
        }

        blocks.swap(first, last);
        first += 1;
        last -= 1;
    }

    let mut checksum = 0;
    for (i, block) in blocks.into_iter().enumerate() {
        let Block::Used(id) = block else {
            break;
        };
        checksum += i * id as usize;
    }

    println!("{checksum}");

    Ok(())
}
