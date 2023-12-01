use anyhow::Result;
use fn_cache::{FnCache, HashCache};
use std::collections::HashMap;

type Wire = String;

#[derive(Hash, PartialEq, Eq)]
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
    let mut gates = input
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
            (output.to_string(), gate)
        })
        .collect::<HashMap<_, _>>();

    gates
        .entry(String::from("b"))
        .and_modify(|v| *v = Provide(Amount(16076)));

    let wire_a = "a".into();

    let mut cache = HashCache::<&Input, u16>::recursive(|cache, input| match input {
        Amount(amount) => *amount,
        Wired(wire) => {
            let gate = gates.get(wire).unwrap();
            let res: u16 = match gate {
                Provide(i) => *cache.get(i),
                And(a, b) => *cache.get(a) & *cache.get(b),
                Or(a, b) => *cache.get(a) | *cache.get(b),
                LeftShift(w, a) => *cache.get(w) << *cache.get(a),
                RightShift(w, a) => *cache.get(w) >> *cache.get(a),
                Not(w) => !*cache.get(w),
            };
            res
        }
    });

    let a = *cache.get(&wire_a);

    println!("{}", a);

    Ok(())
}
