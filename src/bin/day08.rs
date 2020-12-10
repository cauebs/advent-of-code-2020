#![feature(str_split_once)]

use std::{collections::HashSet, str::FromStr};

#[derive(Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, val) = s.split_once(" ").ok_or(())?;
        let val = val.parse().map_err(|_| ())?;

        match op {
            "acc" => Ok(Self::Acc(val)),
            "jmp" => Ok(Self::Jmp(val)),
            "nop" => Ok(Self::Nop(val)),
            _ => Err(()),
        }
    }
}

fn execute(program: &[Instruction]) -> Result<i32, i32> {
    let mut accumulator = 0;
    let mut program_counter = 0;
    let mut executed_lines = HashSet::new();

    while program_counter < program.len() as i32 {
        if !executed_lines.insert(program_counter) {
            return Err(accumulator);
        }

        match program[program_counter as usize] {
            Instruction::Acc(val) => accumulator += val,
            Instruction::Jmp(val) => {
                program_counter += val;
                continue;
            }
            Instruction::Nop(_) => {}
        }

        program_counter += 1;
    }

    Ok(accumulator)
}

struct Patch {
    corrupted_instruction: usize,
    correct_output: i32,
}

fn fix_program(program: &[Instruction]) -> Option<Patch> {
    for i in 0..program.len() {
        if let Instruction::Acc(_) = program[i] {
            continue;
        }

        let mut patched_program = program.to_vec();
        patched_program[i] = match patched_program[i] {
            Instruction::Acc(_) => unreachable!(),
            Instruction::Jmp(val) => Instruction::Nop(val),
            Instruction::Nop(val) => Instruction::Jmp(val),
        };

        if let Ok(output) = execute(&patched_program) {
            return Some(Patch {
                corrupted_instruction: i,
                correct_output: output,
            });
        }
    }

    None
}

fn main() {
    let program = include_str!("../../inputs/day8.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Instruction>>();

    println!("{}", execute(&program).unwrap_err());
    println!("{}", fix_program(&program).unwrap().correct_output);
}
