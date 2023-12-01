use anyhow::Result;

enum Register {
    A,
    B,
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "a" => Register::A,
            "b" => Register::B,
            _ => unreachable!(),
        }
    }
}

struct Offset(isize);

impl From<&str> for Offset {
    fn from(value: &str) -> Self {
        let value = value.strip_prefix('+').unwrap_or(value);
        Self(value.parse().unwrap())
    }
}

enum Instruction {
    Halve(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

use Instruction::*;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/23.txt")?;
    let instructions = input
        .lines()
        .map(|line| {
            let (op, params) = line.split_once(' ').unwrap();
            match op {
                "hlf" => Halve(params.into()),
                "tpl" => Triple(params.into()),
                "inc" => Increment(params.into()),
                "jmp" => Jump(params.into()),
                _ => {
                    let (register, offset) = params.split_once(", ").unwrap();
                    match op {
                        "jie" => JumpIfEven(register.into(), offset.into()),
                        "jio" => JumpIfOne(register.into(), offset.into()),
                        _ => unreachable!(),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let mut register_a: usize = 1;
    let mut register_b: usize = 0;
    let mut pc = 0;

    while let Some(instruction) = instructions.get(pc as usize) {
        match instruction {
            Halve(r) => {
                match r {
                    Register::A => register_a /= 2,
                    Register::B => register_b /= 2,
                }
                pc += 1;
            }
            Triple(r) => {
                match r {
                    Register::A => register_a *= 3,
                    Register::B => register_b *= 3,
                }
                pc += 1;
            }
            Increment(r) => {
                match r {
                    Register::A => register_a += 1,
                    Register::B => register_b += 1,
                }
                pc += 1;
            }
            Jump(o) => {
                pc += o.0;
            }
            JumpIfEven(r, o) => {
                let value = match r {
                    Register::A => register_a,
                    Register::B => register_b,
                };

                if value % 2 == 0 {
                    pc += o.0;
                } else {
                    pc += 1;
                }
            }
            JumpIfOne(r, o) => {
                let value = match r {
                    Register::A => register_a,
                    Register::B => register_b,
                };

                if value == 1 {
                    pc += o.0;
                } else {
                    pc += 1;
                }
            }
        }
    }

    println!("{register_b}");

    Ok(())
}
