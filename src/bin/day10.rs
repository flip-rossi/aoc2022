//! Day 10: Cathode-Ray Tube

use aoc22::solve_puzzle;

const FIRST_CYCLE: i32 = 20;
const CYCLE_INTERVAL: i32 = 40;

use Instruction::{Noop, Addx};

enum Instruction {
    Noop,
    Addx(i32),
}
impl Instruction {
    fn duration(&self) -> i32 {
        match self {
            Noop => 1,
            Addx(_) => 2,
        }
    }
}

fn main() -> Result<(),std::io::Error> {
    // Parse input
    let mut instructions = Vec::new();
    let lines = std::io::stdin().lines();
    for l in lines {
        let l_ok = l?;
        let mut split = l_ok.split(' ');
        match split.next() {
            Some(w) => match w {
                "noop" => instructions.push(Noop),
                "addx" => match split.next() {
                    Some(n) => match i32::from_str_radix(n, 10) {
                        Ok(x) => instructions.push(Addx(x)),
                        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Wrong addx argument.")),
                    }
                    None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing addx argument.")),
                },
                _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown instruction.")),
            },
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing instruction.")),
        }
    }

    // Solve
    let answer = solve_puzzle!(instructions);
    println!("{answer}");

    Ok(())
}

//=============== PART 1 ===============//
fn part1(instructions: Vec<Instruction>) -> i32 {
    let mut answer = 0;

    let mut x = 1;
    let mut cycle = 0;
    for instr in instructions {
        for i in (0..instr.duration()).rev() {
            cycle += 1;
            if (cycle - FIRST_CYCLE) % CYCLE_INTERVAL == 0 {
                eprintln!("--- CYCLE {cycle} ---");
                eprintln!("Register: {x}");
                eprintln!("Strength: {}", x * cycle);
                answer += x * cycle;
            }
            if i == 0 {
                match instr {
                    Noop => {},
                    Addx(y) => x += y,
                }
            }
        }

    }

    answer
}

//=============== PART 2 ===============//
#[allow(unused_variables)]
fn part2(instructions: Vec<Instruction>) -> i32 {
    todo!()
}

