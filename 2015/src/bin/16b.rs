use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let mut want = HashMap::new();
    want.insert("children", 3);
    want.insert("cats", 7);
    want.insert("samoyeds", 2);
    want.insert("pomeranians", 3);
    want.insert("akitas", 0);
    want.insert("vizslas", 0);
    want.insert("goldfish", 5);
    want.insert("trees", 3);
    want.insert("cars", 2);
    want.insert("perfumes", 1);

    let input = std::fs::read_to_string("input/16.txt")?;
    let res = input.lines().find_map(|line| {
        let (id, compounds) = line.split_once(": ").unwrap();
        let (_, id) = id.split_once(' ').unwrap();

        if compounds
            .split(", ")
            .map(|compound| compound.split_once(": ").unwrap())
            .any(|(compound, amount)| {
                let amount: i32 = amount.parse().unwrap();
                if let Some(need) = want.get(compound) {
                    if compound == "cats" || compound == "trees" {
                        *need >= amount
                    } else if compound == "pomeranians" || compound == "goldfish" {
                        *need <= amount
                    } else {
                        *need != amount
                    }
                } else {
                    true
                }
            })
        {
            None
        } else {
            Some(id)
        }
    });

    let Some(res) = res else { unreachable!() };

    println!("{res}");

    Ok(())
}
