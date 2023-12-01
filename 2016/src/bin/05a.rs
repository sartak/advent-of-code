use anyhow::Result;

fn main() -> Result<()> {
    let input = "uqwqemis";

    let password = (0..)
        .filter_map(|i| {
            let key = format!("{input}{i}");
            let digest = md5::compute(key);
            let digest = format!("{:x}", digest);
            if digest.starts_with("00000") {
                Some(digest.as_bytes()[5] as char)
            } else {
                None
            }
        })
        .take(8)
        .collect::<String>();

    println!("{password}");

    Ok(())
}
