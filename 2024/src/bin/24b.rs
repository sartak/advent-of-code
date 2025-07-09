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
enum Wire<'a> {
    Constant(bool),
    Calculated(bool),
    Gate(&'a str, Operation, &'a str),
}
use Wire::*;

fn inject<'a>(wires: &mut HashMap<&'a str, Wire<'a>>, is_x: bool, value: u64) {
    for (&name, wire) in wires.iter_mut() {
        let index = if is_x {
            let Some(i) = name.strip_prefix('x') else {
                continue;
            };
            i
        } else {
            let Some(i) = name.strip_prefix('y') else {
                continue;
            };
            i
        };

        let index: u64 = index.parse().unwrap();
        let value = (value >> index) & 1 == 1;
        *wire = Calculated(value);
    }
}

fn swap_wires<'a>(wires: &mut HashMap<&'a str, Wire<'a>>, a: &'a str, b: &'a str) {
    let wire_a = wires.remove(a).unwrap();
    let wire_b = wires.remove(b).unwrap();
    wires.insert(a, wire_b);
    wires.insert(b, wire_a);
}

fn simulation<'a>(mut wires: HashMap<&'a str, Wire<'a>>, zwires: &[&str]) -> Option<u64> {
    'resolve: loop {
        let mut calculated = Vec::new();

        for &name in wires.keys() {
            let wire = wires.get(name).unwrap();
            let Gate(left, op, right) = wire else {
                continue;
            };

            let left = wires.get(left).unwrap();
            let right = wires.get(right).unwrap();

            let (left, right) = match (left, right) {
                (Gate(_, _, _), _) => continue,
                (_, Gate(_, _, _)) => continue,
                (Constant(l) | Calculated(l), Constant(r) | Calculated(r)) => (*l, *r),
            };

            let value = match op {
                And => left && right,
                Or => left || right,
                Xor => left != right,
            };

            calculated.push((name, Calculated(value)));
        }

        if calculated.is_empty() {
            return None;
        }

        let mut check = false;
        for (name, wire) in calculated {
            wires.insert(name, wire);
            if name.starts_with('z') {
                check = true;
            }
        }

        if check {
            let mut answer = 0;
            for &name in zwires {
                let wire = wires.get(name).unwrap();
                let value = match wire {
                    Calculated(v) => *v,
                    Constant(v) => *v,
                    Gate(_, _, _) => continue 'resolve,
                };

                answer <<= 1;
                answer |= if value { 1 } else { 0 };
            }

            return Some(answer);
        }
    }
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

    let zwires = wires
        .keys()
        .filter(|name| name.starts_with('z'))
        .copied()
        .sorted()
        .rev()
        .collect_vec();

    let max = (1 << (zwires.len() - 1)) - 1;

    swap_wires(&mut wires, "z12", "vdc");
    swap_wires(&mut wires, "z21", "nhn");
    swap_wires(&mut wires, "tvb", "khg");
    swap_wires(&mut wires, "z33", "gst");

    for z in 2..45 {
        check_z(&wires, z)?;
    }

    println!("digraph {{");
    for (name, wire) in &wires {
        if name.starts_with('x') {
            println!("{name} [group=x] [color=purple]");
        } else if name.starts_with('y') {
            println!("{name} [group=y] [color=green]");
        } else if name.starts_with('z') {
            println!("{name} [group=z] [color=blue]");
        }

        match wire {
            Calculated(_) | Constant(_) => {}
            Gate(left, op, right) => {
                println!("{left} -> {name} [label={op:?}]");
                println!("{right} -> {name} [label={op:?}]");
            }
        }
    }
    println!("}}");

    for x in 0..=max {
        for y in 0..=max {
            let mut wires = wires.clone();
            inject(&mut wires, true, x);
            inject(&mut wires, false, y);

            let answer = simulation(wires, &zwires);
            match answer {
                None => bail!("unable to solve for x={x}, y={y}"),
                Some(z) if z == x + y => {}
                Some(got) => bail!("expected {x}+{y}={}, got {got}", x + y),
            }
        }
    }

    Ok(())
}
