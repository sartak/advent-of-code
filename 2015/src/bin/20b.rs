use anyhow::Result;

fn main() -> Result<()> {
    let target = 29000000;
    let mut houses = Vec::new();
    let mut upper_bound = None;

    'elf: for elf in 1.. {
        let amount = elf * 11;
        for stop in 1..=50 {
            let house = elf * stop;
            if house >= houses.len() {
                houses.resize(house + 1, 0);
            }
            houses[house] += amount;
            if houses[house] >= target {
                upper_bound = Some(house);
                break 'elf;
            }
        }
    }

    let upper_bound = upper_bound.unwrap();

    houses.iter_mut().for_each(|v| *v = 0);

    for elf in 1..=upper_bound {
        let amount = elf * 11;
        for stop in 1..=50 {
            let house = elf * stop;
            if house > upper_bound {
                break;
            }
            houses[house] += amount;
        }
    }

    let house = houses.into_iter().position(|v| v >= target).unwrap();
    println!("{house}");

    Ok(())
}
