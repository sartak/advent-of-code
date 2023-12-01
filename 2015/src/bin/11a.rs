use anyhow::Result;
use itertools::Itertools;

type Password = [u8; 8];

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn decode(text: &str) -> Password {
    text.chars()
        .map(|p| ASCII_LOWER.iter().position(|c| *c == p).unwrap() as u8)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn encode(pw: &Password) -> String {
    pw.iter().map(|p| ASCII_LOWER[*p as usize]).collect()
}

fn increment(pw: &mut Password) {
    let mut i = 7;
    loop {
        pw[i] += 1;
        if pw[i] < 26 {
            break;
        }
        pw[i] = 0;
        i -= 1;
    }
}

fn has_bad(pw: &Password) -> bool {
    pw.iter().any(|p| matches!(p, 8 | 11 | 14))
}

fn increasing(pw: &Password) -> bool {
    pw.iter()
        .tuple_windows()
        .any(|(a, b, c)| *a + 1 == *b && *b + 1 == *c)
}

fn repeat(pw: &Password) -> bool {
    match pw.iter().tuple_windows().position(|(a, b)| *a == *b) {
        None => false,
        Some(first) => pw
            .iter()
            .skip(first + 2)
            .tuple_windows()
            .any(|(a, b)| *a == *b),
    }
}

fn main() -> Result<()> {
    let mut password = decode("hepxcrrq");

    loop {
        increment(&mut password);
        if has_bad(&password) {
            continue;
        }
        if !increasing(&password) {
            continue;
        }
        if !repeat(&password) {
            continue;
        }
        break;
    }

    println!("{}", encode(&password));

    Ok(())
}
