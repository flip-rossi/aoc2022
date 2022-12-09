//! Day 9: Rope Bridge

use std::collections::HashSet;

use aoc22::solve_puzzle;

const LEFT:  (i32,i32) = (-1, 0);
const RIGHT: (i32,i32) = ( 1, 0);
const UP:    (i32,i32) = ( 0, 1);
const DOWN:  (i32,i32) = ( 0,-1);

struct Motion {
    dir: (i32, i32),
    amount: i32,
}
impl Motion {
    fn new(dir: (i32, i32), amount: i32) -> Self {
        Self { dir, amount }
    }
}

fn main() {
    // Parse input
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
const KNOTS_N: usize = 10;

fn part2(motions: Vec<Motion>) -> usize {
    let mut knots = [(0,0); KNOTS_N];

    let mut tail_positions = HashSet::new();
    tail_positions.insert(knots[KNOTS_N-1]);
    for m in motions {
        for _ in 0..m.amount {
            knots[0].0 += m.dir.0; knots[0].1 += m.dir.1;
            for i in 1..knots.len() {
                let dist = (knots[i-1].0 - knots[i].0, knots[i-1].1 - knots[i].1);
                if dist.0.abs() >= 2 || dist.1.abs() >= 2 {
                    knots[i].0 += dist.0.signum(); knots[i].1 += dist.1.signum();
                }
            }
            tail_positions.insert(knots[KNOTS_N-1]);
        }
    }

    tail_positions.len()
}

