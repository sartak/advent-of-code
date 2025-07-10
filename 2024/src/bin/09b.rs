use anyhow::Result;

#[derive(Debug)]
enum Block {
    Used((usize, u32)),
    Free(usize),
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/09-example.txt"
    } else {
        "input/09.txt"
    })?;

    let mut blocks = Vec::new();
    let mut is_free = false;
    let mut id = 0;
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        let size = c.to_digit(10).unwrap() as usize;

        if size > 0 {
            blocks.push(if is_free {
                Block::Free(size)
            } else {
                Block::Used((size, id))
            });
        }

        is_free = !is_free;
        if !is_free {
            id += 1;
        }
    }

    id += 1;

    while let Some(new) = id.checked_sub(1) {
        id = new;

        let mut used = None;
        for (i, block) in blocks.iter().enumerate() {
            if let Block::Used((size, b)) = block {
                if *b == id {
                    used = Some((*size, i));
                    break;
                }
            }
        }

        let Some((size, i)) = used else {
            continue;
        };

        let mut empty = None;
        for (j, block) in blocks.iter().enumerate() {
            if j > i {
                break;
            }
            if let Block::Free(s) = block {
                if *s >= size {
                    empty = Some((*s, j));
                    break;
                }
            }
        }

        let Some((free, j)) = empty else {
            continue;
        };

        blocks.swap(i, j);

        blocks[i] = Block::Free(size);
        if free > size {
            blocks.insert(j + 1, Block::Free(free - size));
        }
    }

    let mut checksum = 0;
    let mut pos = 0;
    for block in blocks.into_iter() {
        let (size, id) = match block {
            Block::Used((s, id)) => (s, Some(id)),
            Block::Free(s) => (s, None),
        };

        for _ in 0..size {
            if let Some(id) = id {
                checksum += pos * id as usize;
            }
            pos += 1;
        }
    }

    println!("{checksum}");

    Ok(())
}
