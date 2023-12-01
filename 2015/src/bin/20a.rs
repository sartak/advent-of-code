use anyhow::Result;
use factor::factor_include::factor_include;

fn main() -> Result<()> {
    let target = 2900000; // removed trailing 0
    for i in 1.. {
        let presents: i64 = factor_include(i).into_iter().sum();
        if presents >= target {
            println!("{i}");
            break;
        }
    }

    Ok(())
}
