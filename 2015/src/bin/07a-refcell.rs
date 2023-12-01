use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;

type Wire = String;

enum Input {
    Amount(u16),
    Wired(Wire),
}
use Input::*;

enum Gate {
    Provide(Input),
    And(Input, Input),
    Or(Input, Input),
    LeftShift(Input, Input),
    RightShift(Input, Input),
    Not(Input),
}
use Gate::*;

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        if let Ok(amount) = value.parse::<u16>() {
            Amount(amount)
        } else {
            Wired(value.to_string())
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/07.txt")?;
    let gates = input
        .lines()
        .map(|line| {
            let Some((op, output)) = line.split_once(" -> ") else {
                unreachable!()
            };
            let words = op.split(' ').collect::<Vec<_>>();
            let gate = match words[..] {
                [provide] => Provide(provide.into()),
                [a, "AND", b] => And(a.into(), b.into()),
                [a, "OR", b] => Or(a.into(), b.into()),
                [input, "LSHIFT", amount] => LeftShift(input.into(), amount.into()),
                [input, "RSHIFT", amount] => RightShift(input.into(), amount.into()),
                ["NOT", input] => Not(input.into()),
                _ => unreachable!(),
            };
            (output.to_string(), RefCell::new(gate))
        })
        .collect::<HashMap<_, _>>();

    fn signal(gates: &HashMap<Wire, RefCell<Gate>>, input: &Input) -> u16 {
        match input {
            Amount(amount) => *amount,
            Wired(wire) => {
                let gate = gates.get(wire).unwrap();
                let res: u16 = match &*gate.borrow() {
                    Provide(i) => signal(gates, i),
                    And(a, b) => signal(gates, a) & signal(gates, b),
                    Or(a, b) => signal(gates, a) | signal(gates, b),
                    LeftShift(w, a) => signal(gates, w) << signal(gates, a),
                    RightShift(w, a) => signal(gates, w) >> signal(gates, a),
                    Not(w) => !signal(gates, w),
                };

                gate.replace(Provide(Amount(res)));

                res
            }
        }
    }

    /*
    let mut wires = gates.keys().collect::<Vec<_>>();
    wires.sort();
    for wire in wires {
        println!("{wire}: {}", signal(&gates, &wire.as_str().into()));
    }
    */

    let wire = "a".into();
    println!("{}", signal(&gates, &wire));

    Ok(())
}
