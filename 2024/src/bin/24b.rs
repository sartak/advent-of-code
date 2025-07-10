use anyhow::{Context, Result, bail};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}
use Operation::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Wire<'a> {
    Constant(bool),
    Calculated(bool),
    Gate(&'a str, Operation, &'a str),
}
use Wire::*;

fn swap_wires<'a>(wires: &mut HashMap<&'a str, Wire<'a>>, a: &'a str, b: &'a str) {
    let wire_a = wires.remove(a).unwrap();
    let wire_b = wires.remove(b).unwrap();
    wires.insert(a, wire_b);
    wires.insert(b, wire_a);
}

fn check_xy_xor<'a>(name: &str, wire: &Wire<'a>) -> Result<()> {
    match wire {
        Calculated(_) => {
            bail!("expected {name} to be gate, got calculated");
        }
        Constant(_) => {
            bail!("expected {name} to be gate, got constant");
        }
        Gate(left_name, op, right_name) => {
            if *op != Xor {
                bail!("expected {name} to be x/y Xor, got {op:?}");
            }

            let has_x = left_name.starts_with('x') || right_name.starts_with('x');
            let has_y = left_name.starts_with('y') || right_name.starts_with('y');

            if !has_x || !has_y {
                bail!("expected {name} to be x/y Xor, got {left_name} and {right_name}");
            }

            Ok(())
        }
    }
}

fn check_xy_and<'a>(name: &str, wire: &Wire<'a>) -> Result<()> {
    match wire {
        Calculated(_) => {
            bail!("expected {name} to be gate, got calculated");
        }
        Constant(_) => {
            bail!("expected {name} to be gate, got constant");
        }
        Gate(left_name, op, right_name) => {
            if *op != And {
                bail!("expected {name} to be x/y And, got {op:?}");
            }

            let has_x = left_name.starts_with('x') || right_name.starts_with('x');
            let has_y = left_name.starts_with('y') || right_name.starts_with('y');

            if !has_x || !has_y {
                bail!("expected {name} to be x/y And, got {left_name} and {right_name}");
            }

            Ok(())
        }
    }
}

fn check_alpha<'a>(wires: &HashMap<&'a str, Wire<'a>>, name: &str, wire: &Wire<'a>) -> Result<()> {
    match wire {
        Calculated(_) => {
            bail!("expected {name} to be gate, got calculated");
        }
        Constant(_) => {
            bail!("expected {name} to be gate, got constant");
        }
        Gate(left_name, op, right_name) => {
            if *op != Or {
                bail!("expected {name} to be non-x/y Or, got {op:?}");
            }

            let has_x = left_name.starts_with('x') || right_name.starts_with('x');
            let has_y = left_name.starts_with('y') || right_name.starts_with('y');

            if has_x || has_y {
                bail!("expected {name} to be non-x/y, got {left_name} and {right_name}");
            }

            let left_wire = wires.get(left_name).unwrap();
            let right_wire = wires.get(right_name).unwrap();

            let left_xy = check_xy_and(left_name, left_wire)
                .with_context(|| format!("incorrect left branch in {name}"));
            let right_beta = check_beta(right_name, right_wire)
                .with_context(|| format!("incorrect right branch in {name}"));

            if left_xy.is_err() && right_beta.is_err() {
                check_beta(left_name, left_wire)
                    .with_context(|| format!("incorrect left branch in {name}"))?;
                check_xy_and(right_name, right_wire)
                    .with_context(|| format!("incorrect right branch in {name}"))?;
            } else {
                left_xy?;
                right_beta?;
            }

            Ok(())
        }
    }
}

fn check_beta<'a>(name: &str, wire: &Wire<'a>) -> Result<()> {
    match wire {
        Calculated(_) => {
            bail!("expected {name} to be gate, got calculated");
        }
        Constant(_) => {
            bail!("expected {name} to be gate, got constant");
        }
        Gate(left_name, op, right_name) => {
            if *op != And {
                bail!("expected {name} to be And, got {op:?}");
            }

            let has_x = left_name.starts_with('x') || right_name.starts_with('x');
            let has_y = left_name.starts_with('y') || right_name.starts_with('y');

            if has_x || has_y {
                bail!("expected {name} to be non-x/y, got {left_name} and {right_name}");
            }

            Ok(())
        }
    }
}

fn check_z<'a>(wires: &HashMap<&'a str, Wire<'a>>, z: usize) -> Result<()> {
    let name = format!("z{:0>2}", z);
    let wire = wires.get(&name.as_ref()).unwrap();
    match wire {
        Calculated(_) => {
            bail!("expected {name} to be gate, got calculated");
        }
        Constant(_) => {
            bail!("expected {name} to be gate, got constant");
        }
        Gate(left_name, op, right_name) => {
            if *op != Xor {
                bail!("expected {name} to be Xor, got {op:?}");
            }

            let left_wire = wires.get(left_name).unwrap();
            let right_wire = wires.get(right_name).unwrap();

            if left_name.starts_with('x') || left_name.starts_with('y') {
                bail!("unexpected {left_name} in {name}");
            }

            if right_name.starts_with('x') || left_name.starts_with('y') {
                bail!("unexpected {right_name} in {name}");
            }

            let left_alpha = check_alpha(wires, left_name, left_wire)
                .with_context(|| format!("incorrect left branch in {name}"));
            let right_xy_xor = check_xy_xor(right_name, right_wire)
                .with_context(|| format!("incorrect right branch in {name}"));

            if left_alpha.is_err() && right_xy_xor.is_err() {
                check_xy_xor(left_name, left_wire)
                    .with_context(|| format!("incorrect left branch in {name}"))?;
                check_alpha(wires, right_name, right_wire)
                    .with_context(|| format!("incorrect left branch in {name}"))?;
            } else {
                left_alpha?;
                right_xy_xor?;
            }

            Ok(())
        }
    }
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    let input = std::fs::read_to_string("input/24-example.txt")?;
    #[cfg(not(debug_assertions))]
    let input = std::fs::read_to_string("input/24.txt")?;

    let const_rx = Regex::new(r"^(\w+): (0|1)$").unwrap();
    let gate_rx = Regex::new(r"^(\w+) (AND|OR|XOR) (\w+) -> (\w+)$").unwrap();

    let mut wires = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = const_rx.captures(line) {
            let (_, [wire, bit]) = caps.extract();
            let bit = match bit {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            };
            wires.insert(wire, Constant(bit));
            continue;
        }

        if let Some(caps) = gate_rx.captures(line) {
            let (_, [left, op, right, output]) = caps.extract();
            let op = match op {
                "AND" => And,
                "OR" => Or,
                "XOR" => Xor,
                _ => unreachable!(),
            };
            wires.insert(output, Gate(left, op, right));
            continue;
        }

        panic!("Unhandled input line: {line}");
    }

    let swaps = [
        ("z12", "vdc"),
        ("z21", "nhn"),
        ("tvb", "khg"),
        ("z33", "gst"),
    ];
    for (a, b) in swaps {
        swap_wires(&mut wires, a, b);
    }

    for z in 2..45 {
        check_z(&wires, z)?;
    }

    println!(
        "{}",
        swaps.iter().flat_map(|(a, b)| [a, b]).sorted().join(",")
    );

    Ok(())
}
