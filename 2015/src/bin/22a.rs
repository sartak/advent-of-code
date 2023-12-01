use anyhow::Result;
use std::cmp::max;
use std::collections::BinaryHeap;

#[derive(PartialEq)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}
use Spell::*;

impl Spell {
    fn can_cast(&self, game: &Game) -> bool {
        if self.cost() > game.mana {
            return false;
        }

        match self {
            Missile => true,
            Drain => true,
            Shield => game.effects.shield == 0,
            Poison => game.effects.poison == 0,
            Recharge => game.effects.recharge == 0,
        }
    }

    fn cost(&self) -> i32 {
        match self {
            Missile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229,
        }
    }
}

#[derive(PartialEq, Eq, Default, Clone)]
struct Effects {
    shield: usize,
    poison: usize,
    recharge: usize,
}

#[derive(PartialEq, Eq, Clone)]
struct Game {
    mana: i32,
    player_hp: i32,
    boss_hp: i32,
    effects: Effects,
    spent: i32,
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.spent.partial_cmp(&self.spent)
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.spent.cmp(&self.spent)
    }
}

fn main() -> Result<()> {
    let mana_start = 500;
    let player_hp_start = 50;
    let boss_hp_start = 71;
    let boss_damage = 10;

    let mut queue = BinaryHeap::new();
    queue.push(Game {
        mana: mana_start,
        player_hp: player_hp_start,
        boss_hp: boss_hp_start,
        effects: Default::default(),
        spent: 0,
    });

    let spent = loop {
        let mut game = queue.pop().unwrap();

        if game.boss_hp <= 0 {
            break game.spent;
        }

        if game.effects.shield > 0 {
            game.effects.shield -= 1;
        }

        if game.effects.poison > 0 {
            game.effects.poison -= 1;
            game.boss_hp -= 3;
            // can't short-circuit here because player must cast a spell
        }

        if game.effects.recharge > 0 {
            game.effects.recharge -= 1;
            game.mana += 101;
        }

        for spell in [Missile, Drain, Shield, Poison, Recharge] {
            if !spell.can_cast(&game) {
                continue;
            }

            let mut game = game.clone();
            let cost = spell.cost();
            game.mana -= cost;
            game.spent += cost;

            match spell {
                Missile => {
                    game.boss_hp -= 4;
                }
                Drain => {
                    game.boss_hp -= 2;
                    game.player_hp += 2;
                }
                Shield => {
                    game.effects.shield = 6;
                }
                Poison => {
                    game.effects.poison = 6;
                }
                Recharge => {
                    game.effects.recharge = 5;
                }
            };

            // short circuit
            if game.boss_hp <= 0 {
                queue.push(game);
                continue;
            }

            // boss turn

            let armor = if game.effects.shield > 0 {
                game.effects.shield -= 1;
                7
            } else {
                0
            };

            if game.effects.poison > 0 {
                game.effects.poison -= 1;
                game.boss_hp -= 3;
                if game.boss_hp <= 0 {
                    queue.push(game);
                    continue;
                }
            }

            if game.effects.recharge > 0 {
                game.effects.recharge -= 1;
                game.mana += 101;
            }

            let damage = max(1, boss_damage - armor);
            game.player_hp -= damage;

            if game.player_hp > 0 {
                queue.push(game);
            }
        }
    };

    println!("{spent}");

    Ok(())
}
