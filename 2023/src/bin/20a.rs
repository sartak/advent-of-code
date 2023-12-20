use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

enum Module<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcaster,
}
use Module::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/20.txt")?;

    let mut modules: HashMap<&str, (Module, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let (module, targets) = line.split_once(" -> ").unwrap();

            let (name, module) = if let Some(name) = module.strip_prefix('&') {
                (name, Conjunction(HashMap::new()))
            } else if let Some(name) = module.strip_prefix('%') {
                (name, FlipFlop(false))
            } else if module == "broadcaster" {
                (module, Broadcaster)
            } else {
                panic!("invalid module {module}");
            };

            let targets = targets.split(", ").collect_vec();
            (name, (module, targets))
        })
        .collect();

    for (name, upstreams) in modules
        .iter()
        .flat_map(|(name, (_, targets))| {
            targets.iter().filter_map(|&target| {
                matches!(modules.get(target), Some((Conjunction(_), _))).then_some((target, *name))
            })
        })
        .into_group_map()
    {
        let Some((Conjunction(memory), _)) = modules.get_mut(name) else {
            unreachable!()
        };
        memory.extend(upstreams.into_iter().map(|name| (name, false)));
    }

    let (mut ans_lo, mut ans_hi) = (0, 0);
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", false));

        while let Some((from, name, pulse)) = queue.pop_front() {
            if pulse {
                ans_hi += 1;
            } else {
                ans_lo += 1;
            }

            let Some((module, targets)) = modules.get_mut(name) else {
                continue;
            };

            let pulse = match module {
                FlipFlop(state) => {
                    if pulse {
                        continue;
                    }
                    *state = !*state;
                    *state
                }
                Conjunction(memory) => {
                    let cell = memory.get_mut(from).unwrap();
                    *cell = pulse;
                    !memory.values().all(|&v| v)
                }
                Broadcaster => pulse,
            };

            for target in targets {
                queue.push_back((name, target, pulse));
            }
        }
    }

    let ans = ans_lo * ans_hi;
    println!("{ans}");
    Ok(())
}
