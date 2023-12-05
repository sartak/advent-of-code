use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use regex::Regex;

type Bot = usize;
type Value = usize;
type Output = usize;

#[derive(Clone)]
enum Target {
    Bot(Bot),
    Output(Output),
}

#[derive(Clone)]
struct Instruction(Target, Target);

impl From<&str> for Target {
    fn from(value: &str) -> Self {
        let (kind, amount) = value.split_once(' ').unwrap();
        let amount = amount.parse().unwrap();
        match kind {
            "bot" => Target::Bot(amount),
            "output" => Target::Output(amount),
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/10.txt")?;
    let value_rx = Regex::new(r"^value (\d+) goes to bot (\d+)$")?;
    let give_rx = Regex::new(
        r"^bot (\d+) gives low to (bot \d+|output \d+) and high to (bot \d+|output \d+)",
    )?;

    let mut outputs: HashMap<Output, Value> = HashMap::new();
    let mut bots: HashMap<Bot, (Option<Value>, Option<Value>, Option<Instruction>)> =
        HashMap::new();
    let mut ready = VecDeque::new();

    for line in input.lines() {
        if let Some(caps) = value_rx.captures(line) {
            let value = caps.get(1).unwrap().as_str().parse()?;
            let bot = caps.get(2).unwrap().as_str().parse()?;
            let res = bots
                .entry(bot)
                .and_modify(|v| {
                    if v.0.is_some() {
                        v.1 = Some(value)
                    } else {
                        v.0 = Some(value)
                    }
                })
                .or_insert((Some(value), None, None));
            if res.1.is_some() {
                ready.push_back(bot);
            }
        } else {
            let caps = give_rx.captures(line).unwrap();
            let bot = caps.get(1).unwrap().as_str().parse()?;
            let instruction = Instruction(
                caps.get(2).unwrap().as_str().into(),
                caps.get(3).unwrap().as_str().into(),
            );

            bots.entry(bot)
                .and_modify(|v| v.2 = Some(instruction.clone()))
                .or_insert((None, None, Some(instruction)));
        }
    }

    while let Some(bot) = ready.pop_front() {
        if let Some((first, second, instruction)) = bots.remove(&bot) {
            let first = first.unwrap();
            let second = second.unwrap();
            let Instruction(low_target, high_target) = instruction.unwrap();

            let (low, high) = if first < second {
                (first, second)
            } else {
                (second, first)
            };

            for (value, target) in [(low, low_target), (high, high_target)] {
                match target {
                    Target::Bot(bot) => {
                        let res = bots
                            .entry(bot)
                            .and_modify(|v| {
                                if v.0.is_some() {
                                    v.1 = Some(value)
                                } else {
                                    v.0 = Some(value)
                                }
                            })
                            .or_insert((Some(value), None, None));
                        if res.1.is_some() {
                            ready.push_back(bot);
                        }
                    }
                    Target::Output(output) => {
                        outputs.insert(output, value);
                    }
                }
            }
        }
    }

    let value = outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap();
    println!("{value}");

    Ok(())
}
