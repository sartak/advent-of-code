use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(if cfg!(debug_assertions) {
        "input/17-example.txt"
    } else {
        "input/17.txt"
    })?;

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;
    let mut program: Vec<u64> = Vec::new();

    let rx_a = Regex::new(r"^Register A: (\d+)$").unwrap();
    let rx_b = Regex::new(r"^Register B: (\d+)$").unwrap();
    let rx_c = Regex::new(r"^Register C: (\d+)$").unwrap();
    let rx_pgm = Regex::new(r"^Program: (.+)$").unwrap();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = rx_a.captures(line) {
            let (_, [value]) = caps.extract();
            a = value.parse().unwrap();
            continue;
        }

        if let Some(caps) = rx_b.captures(line) {
            let (_, [value]) = caps.extract();
            b = value.parse().unwrap();
            continue;
        }

        if let Some(caps) = rx_c.captures(line) {
            let (_, [value]) = caps.extract();
            c = value.parse().unwrap();
            continue;
        }

        if let Some(caps) = rx_pgm.captures(line) {
            let (_, [value]) = caps.extract();
            program = value
                .split(',')
                .map(|value| value.parse().unwrap())
                .collect();
            continue;
        }

        panic!("Unexpected line {line}");
    }

    let mut ip = 0;
    let mut output = Vec::new();

    loop {
        let Some(&opcode) = program.get(ip) else {
            break;
        };

        let Some(&raw_operand) = program.get(ip + 1) else {
            unreachable!();
        };

        let literal_operand = raw_operand;

        let combo_operand = || match raw_operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!("invalid combo operand {raw_operand}"),
        };

        let mut jumped = false;

        match opcode {
            // adv (division)
            0 => {
                let numerator = a;
                let denominator = 1 << combo_operand();
                let value = numerator / denominator;
                a = value;
            }
            // bxl (bitwise XOR)
            1 => {
                let value = b ^ literal_operand;
                b = value;
            }
            // bst (modulo 8)
            2 => {
                let value = combo_operand() % 8;
                b = value;
            }
            // jnz (jump not zero)
            3 => {
                if a != 0 {
                    ip = literal_operand as usize;
                    jumped = true;
                }
            }
            // bxc (bitwise XOR)
            4 => {
                let value = b ^ c;
                b = value;
            }
            // out
            5 => {
                let value = combo_operand() % 8;
                output.push(value);
            }
            // bdv (division)
            6 => {
                let numerator = a;
                let denominator = 1 << combo_operand();
                let value = numerator / denominator;
                b = value;
            }
            // cdv division
            7 => {
                let numerator = a;
                let denominator = 1 << combo_operand();
                let value = numerator / denominator;
                c = value;
            }

            _ => unreachable!("invalid opcode {opcode}"),
        }

        if !jumped {
            ip += 2;
        }
    }

    println!("{}", output.into_iter().map(|i| format!("{i}")).join(","));

    Ok(())
}
