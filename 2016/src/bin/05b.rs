use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = "uqwqemis";
    let mut password = vec!['_'; 8];

    (0..)
        .filter_map(|i| {
            let key = format!("{input}{i}");
            let digest = md5::compute(key);
            let digest = format!("{:x}", digest);
            if digest.starts_with("00000") {
                let position = digest.as_bytes()[5] as char;
                if ('0'..='7').contains(&position) {
                    let position = position as u8 - b'0';
                    let character = digest.as_bytes()[6] as char;
                    return Some((position, character));
                }
            }
            None
        })
        .unique_by(|(p, _)| *p)
        .take(8)
        .for_each(|(position, character)| {
            password[position as usize] = character;
        });

    println!("{}", password.into_iter().map(String::from).join(""));

    Ok(())
}
