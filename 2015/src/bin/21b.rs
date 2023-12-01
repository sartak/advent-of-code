use anyhow::Result;
use std::{cmp::max, collections::VecDeque};

#[derive(Debug)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

struct Equipment<'a> {
    weapon: &'a Item,
    armor: Option<&'a Item>,
    ring1: Option<&'a Item>,
    ring2: Option<&'a Item>,
}

impl<'a> Equipment<'a> {
    fn cost(&self) -> i32 {
        let equipment = [Some(self.weapon), self.armor, self.ring1, self.ring2];
        equipment
            .iter()
            .map(|i| i.map(|i| i.cost).unwrap_or(0))
            .sum()
    }

    fn stats(&self) -> (i32, i32) {
        let equipment = [Some(self.weapon), self.armor, self.ring1, self.ring2];
        let damage = equipment
            .iter()
            .map(|i| i.map(|i| i.damage).unwrap_or(0))
            .sum();
        let armor = equipment
            .iter()
            .map(|i| i.map(|i| i.armor).unwrap_or(0))
            .sum();
        (damage, armor)
    }
}

fn wins_fight(equipment: &Equipment, boss_hp: i32, boss_damage: i32, boss_armor: i32) -> bool {
    let mut my_hp = 100;
    let mut boss_hp = boss_hp;
    let (my_damage, my_armor) = equipment.stats();

    let my_attack = max(1, my_damage - boss_armor);
    let boss_attack = max(1, boss_damage - my_armor);

    loop {
        boss_hp -= my_attack;
        if boss_hp <= 0 {
            break true;
        }

        my_hp -= boss_attack;
        if my_hp <= 0 {
            break false;
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/21.txt")?;

    #[derive(PartialEq)]
    enum Mode {
        Weapons,
        Armors,
        Rings,
        Boss,
    }
    let mut mode = Mode::Weapons;

    let mut weapons = Vec::new();
    let mut armors = Vec::new();
    let mut rings = Vec::new();
    let mut boss_hp = None;
    let mut boss_damage = None;
    let mut boss_armor = None;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("Weapons:") {
            continue;
        } else if mode == Mode::Weapons && line.starts_with("Armor:") {
            mode = Mode::Armors;
            continue;
        } else if mode == Mode::Armors && line.starts_with("Rings:") {
            mode = Mode::Rings;
            continue;
        } else if mode == Mode::Rings && line.starts_with("Hit Points:") {
            mode = Mode::Boss;
        }

        match mode {
            Mode::Boss => {
                let (stat, value) = line.split_once(": ").unwrap();
                let value = value.parse::<i32>().unwrap();
                match stat {
                    "Hit Points" => boss_hp = Some(value),
                    "Damage" => boss_damage = Some(value),
                    "Armor" => boss_armor = Some(value),
                    _ => unreachable!(),
                }
            }
            _ => {
                let mut words = line
                    .split_whitespace()
                    .map(String::from)
                    .collect::<VecDeque<_>>();
                while words.len() > 4 {
                    let first = words.pop_front().unwrap();
                    let second = words.pop_front().unwrap();
                    words.push_front(format!("{first} {second}",));
                }
                let name = words[0].clone();
                let cost = words[1].parse().unwrap();
                let damage = words[2].parse().unwrap();
                let armor = words[3].parse().unwrap();
                let item = Item {
                    name,
                    cost,
                    damage,
                    armor,
                };

                match mode {
                    Mode::Weapons => weapons.push(item),
                    Mode::Armors => armors.push(item),
                    Mode::Rings => rings.push(item),
                    _ => unreachable!(),
                }
            }
        }
    }

    let boss_hp = boss_hp.unwrap();
    let boss_damage = boss_damage.unwrap();
    let boss_armor = boss_armor.unwrap();

    let mut best = None;

    for weapon in &weapons {
        for armor in armors.iter().map(Some).chain(std::iter::once(None)) {
            for ring1 in rings.iter().map(Some).chain(std::iter::once(None)) {
                for ring2 in rings.iter().map(Some).chain(std::iter::once(None)) {
                    if matches!((ring1, ring2), (Some(r1), Some(r2)) if std::ptr::eq(r1, r2)) {
                        continue;
                    }

                    let equipment = Equipment {
                        weapon,
                        armor,
                        ring1,
                        ring2,
                    };

                    let cost = equipment.cost();
                    if let Some(best) = best {
                        if cost < best {
                            continue;
                        }
                    }

                    if !wins_fight(&equipment, boss_hp, boss_damage, boss_armor) {
                        best = Some(cost);
                    }
                }
            }
        }
    }

    let best = best.unwrap();
    println!("{best}");

    Ok(())
}
