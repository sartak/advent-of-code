use anyhow::Result;

fn main() -> Result<()> {
    let key = "ckczppom";
    for i in 1.. {
        let text = format!("{key}{i}");
        let digest = md5::compute(text);
        let digest = format!("{:x}", digest);
        if digest.starts_with("00000") {
            println!("{i} {digest}");
            break;
        }
    }

    Ok(())
}
