//! Day 9: Rope Bridge

use std::collections::HashSet;

use aoc22::solve_puzzle;

struct Motion {
    dir: (i32, i32),
    amount: i32,
}
impl Motion {
    fn new(dir: (i32, i32), amount: i32) -> Self {
        Self { dir, amount }
    }
}

// const LEFT: &str = "L";
const LEFT:  (i32,i32) = (-1, 0);
const RIGHT: (i32,i32) = ( 1, 0);
const UP:    (i32,i32) = ( 0, 1);
const DOWN:  (i32,i32) = ( 0,-1);

fn main() {
    // Parse arguments
    let input = std::io::stdin().lines();
    let motions: Vec<Motion> = input.map(|s| {
        let line = s.unwrap();
        let words: Vec<&str> = line.split(" ").collect();
        match words[0] {
            "L" => Motion::new(LEFT,  i32::from_str_radix(words[1], 10).unwrap()),
            "R" => Motion::new(RIGHT, i32::from_str_radix(words[1], 10).unwrap()),
            "U" => Motion::new(UP,    i32::from_str_radix(words[1], 10).unwrap()),
            "D" => Motion::new(DOWN,  i32::from_str_radix(words[1], 10).unwrap()),
            _ => panic!("Input parsing error"),
        }
    }).collect();

    // Solve
    let answer = solve_puzzle!(motions);
    println!("{answer}")
}

//=============== PART 1 ===============//
fn part1(motions: Vec<Motion>) -> usize {
    let mut tail = (0,0);
    let mut head = (0,0);

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert(tail);
    for m in motions {
        for _ in 0..m.amount {
            head.0 += m.dir.0; head.1 += m.dir.1;
            let dist = (head.0 - tail.0, head.1 - tail.1);
            if dist.0.abs() >= 2 || dist.1.abs() >= 2 {
                tail.0 += dist.0.signum(); tail.1 += dist.1.signum();
                tail_positions.insert(tail);
            }
        }
    }

    tail_positions.len()
}

//=============== PART 2 ===============//
fn part2(motions: Vec<Motion>) -> usize {
    todo!()
}

