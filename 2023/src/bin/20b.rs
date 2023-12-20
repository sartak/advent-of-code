use anyhow::Result;
use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum Module {
    FlipFlop,
    Conjunction,
    Broadcaster,
}
use Module::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/20.txt")?;

    let mut modules = HashMap::new();

    for line in input.lines() {
        let (module, targets) = line.split_once(" -> ").unwrap();

        let (name, module) = if let Some(module) = module.strip_prefix('&') {
            (module, Conjunction)
        } else if let Some(module) = module.strip_prefix('%') {
            (module, FlipFlop)
        } else if module == "broadcaster" {
            (module, Broadcaster)
        } else {
            panic!();
        };

        let targets = targets.split(", ").map(String::from).collect_vec();

        let name = String::from(name);

        modules.insert(name.clone(), (module, targets));
    }

    let mut memory = HashMap::new();
    for (name, (_, targets)) in modules.iter() {
        for target in targets.iter() {
            let Some((Conjunction, _)) = modules.get(target) else {
                continue;
            };

            memory
                .entry(target)
                .and_modify(|v: &mut HashMap<&String, bool>| {
                    v.insert(name, false);
                })
                .or_insert(map! { name => false });
        }
    }

    let mut is_high = HashSet::new();

    // this assumes &dg -> rx
    let upstream = "dg";
    let mut mult = HashMap::new();
    let need = memory.get(&String::from(upstream)).unwrap().len();

    'push: for i in 1.. {
        let origin = String::from("broadcaster");
        let mut queue = VecDeque::new();
        queue.push_back((origin.clone(), origin, false));

        while let Some((from, name, pulse)) = queue.pop_front() {
            // will ~never finish
            if name == "rx" && !pulse {
                println!("{i}");
                break 'push;
            }

            if name == upstream && pulse && !mult.contains_key(&from) {
                mult.insert(from.clone(), i as u64);
                if mult.len() == need {
                    break 'push;
                }
            }

            let Some((module, targets)) = modules.get(&name) else {
                continue;
            };

            match module {
                FlipFlop => {
                    if !pulse {
                        let emit = if is_high.contains(&name) {
                            is_high.remove(&name);
                            false
                        } else {
                            is_high.insert(name.clone());
                            true
                        };
                        for target in targets {
                            queue.push_back((name.clone(), target.to_owned(), emit));
                        }
                    }
                }
                Conjunction => {
                    let memory = memory.get_mut(&name).unwrap();
                    let cell = memory.get_mut(&from).unwrap();
                    *cell = pulse;
                    let emit = !memory.values().all(|&v| v);
                    for target in targets {
                        queue.push_back((name.clone(), target.to_owned(), emit));
                    }
                }
                Broadcaster => {
                    let emit = pulse;
                    for target in targets {
                        queue.push_back((name.clone(), target.to_owned(), emit));
                    }
                }
            };
        }
    }

    let ans = mult.into_values().fold(1, lcm);
    println!("{ans}");

    Ok(())
}
