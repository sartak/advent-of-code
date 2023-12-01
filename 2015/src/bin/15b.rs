use anyhow::Result;
use regex::Regex;
use std::cmp::max;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/15.txt")?;
    let regex = Regex::new(
        r"^\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$",
    )?;

    let ingredients = input
        .lines()
        .map(|line| {
            let Some(caps) = regex.captures(line) else {
                unreachable!()
            };
            let (_, [capacity, durability, flavor, texture, calories]) = caps.extract();

            Ok(Ingredient {
                capacity: capacity.parse()?,
                durability: durability.parse()?,
                flavor: flavor.parse()?,
                texture: texture.parse()?,
                calories: calories.parse()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let count = 100;
    let mut best = 0;

    for a in 0..=count {
        let ingredient = &ingredients[0];
        let capacity = ingredient.capacity * a;
        let durability = ingredient.durability * a;
        let flavor = ingredient.flavor * a;
        let texture = ingredient.texture * a;
        let calories = ingredient.calories * a;

        for b in 0..=count - a {
            let ingredient = &ingredients[1];
            let capacity = capacity + ingredient.capacity * b;
            let durability = durability + ingredient.durability * b;
            let flavor = flavor + ingredient.flavor * b;
            let texture = texture + ingredient.texture * b;
            let calories = calories + ingredient.calories * b;

            for c in 0..=count - b {
                let ingredient = &ingredients[2];
                let capacity = capacity + ingredient.capacity * c;
                let durability = durability + ingredient.durability * c;
                let flavor = flavor + ingredient.flavor * c;
                let texture = texture + ingredient.texture * c;
                let calories = calories + ingredient.calories * c;

                let d = count - a - b - c;
                let ingredient = &ingredients[3];
                let capacity = capacity + ingredient.capacity * d;
                let durability = durability + ingredient.durability * d;
                let flavor = flavor + ingredient.flavor * d;
                let texture = texture + ingredient.texture * d;
                let calories = calories + ingredient.calories * d;

                if calories != 500 {
                    continue;
                }

                let capacity = max(capacity, 0);
                let durability = max(durability, 0);
                let flavor = max(flavor, 0);
                let texture = max(texture, 0);

                let score = capacity * durability * flavor * texture;
                if score > best {
                    best = score;
                }
            }
        }
    }

    println!("{best}");

    Ok(())
}
