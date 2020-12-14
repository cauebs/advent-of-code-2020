#![feature(str_split_once)]

use std::{collections::HashMap, convert::TryInto, str::FromStr};

#[derive(Clone, Debug)]
struct Mask([char; 36]);

impl Default for Mask {
    fn default() -> Self {
        Self(['X'; 36])
    }
}

impl Mask {
    fn apply_to_value(&self, mut value: u64) -> u64 {
        for (i, &mask_bit) in self.0.iter().rev().enumerate() {
            match mask_bit {
                '0' => value &= !(1 << i),
                '1' => value |= 1 << i,
                'X' => {}
                _ => panic!(),
            }
        }
        value
    }

    fn apply_to_address(&self, mut address: u64) -> Vec<u64> {
        let mut floating_bits = Vec::new();

        for (i, &mask_bit) in self.0.iter().rev().enumerate() {
            match mask_bit {
                '0' => {}
                '1' => address |= 1 << i,
                'X' => floating_bits.push(i),
                _ => panic!(),
            }
        }

        let mut addresses = vec![address];

        for i in floating_bits {
            let mut new_addresses = Vec::new();
            for address in &addresses {
                new_addresses.push(address & !(1 << i));
                new_addresses.push(address | (1 << i));
            }
            addresses = new_addresses;
        }

        addresses
    }
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .collect::<Vec<_>>()
            .try_into()
            .map(Self)
            .map_err(|_| ())
    }
}

#[derive(Debug)]
enum Instruction {
    UpdateMask(Mask),
    Write { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(" = ").ok_or(())?;

        Ok(match lhs {
            "mask" => Self::UpdateMask(rhs.parse().map_err(|_| ())?),
            s => {
                if &s[0..3] != "mem" {
                    return Err(());
                }
                let address = s[4..s.len() - 1].parse().map_err(|_| ())?;
                let value = rhs.parse().map_err(|_| ())?;
                Self::Write { address, value }
            }
        })
    }
}

type Memory = HashMap<u64, u64>;

fn run(program: &[Instruction], write_rule: impl Fn(&mut Memory, &Mask, u64, u64)) -> Memory {
    let mut memory = HashMap::new();
    let mut mask = Mask::default();

    for instruction in program {
        match instruction {
            Instruction::UpdateMask(new_mask) => mask = new_mask.clone(),
            Instruction::Write { address, value } => {
                write_rule(&mut memory, &mask, *address, *value)
            }
        }
    }

    memory
}

fn main() {
    let program = include_str!("../../inputs/day14.txt")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Instruction>, _>>()
        .unwrap();

    let v1_output = run(&program, |memory, mask, address, value| {
        memory.insert(address, mask.apply_to_value(value));
    })
    .values()
    .sum::<u64>();

    println!("{}", v1_output);

    let v2_output = run(&program, |memory, mask, address, value| {
        for address in mask.apply_to_address(address) {
            memory.insert(address, value);
        }
    })
    .values()
    .sum::<u64>();

    println!("{}", v2_output);
}
