use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
}
use Operation::*;

#[derive(Debug)]
enum Wire<'a> {
    Constant(bool),
    Calculated(bool),
    Gate(&'a str, Operation, &'a str),
}
use Wire::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/24-example.txt"
    } else {
        "input/24.txt"
    })?;

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

        let mut check = false;
        for (name, wire) in calculated {
            wires.insert(name, wire);
            if name.starts_with('z') {
                check = true;
            }
        }

        if check {
            let mut answer = 0u64;
            for &name in &zwires {
                let wire = wires.get(name).unwrap();
                let value = match wire {
                    Calculated(v) => *v,
                    Constant(v) => *v,
                    Gate(_, _, _) => continue 'resolve,
                };

                answer <<= 1;
                answer |= if value { 1 } else { 0 };
            }
            println!("{answer}");
            break;
        }
    }

    Ok(())
}
