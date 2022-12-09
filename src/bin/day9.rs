//! Day 9: Rope Bridge

use std::{collections::HashSet, ops::{Add, Sub, AddAssign}};

use aoc22::solve_puzzle;

const LEFT:  (i32,i32) = (-1, 0);
const RIGHT: (i32,i32) = ( 1, 0);
const UP:    (i32,i32) = ( 0, 1);
const DOWN:  (i32,i32) = ( 0,-1);

const ORIGIN: Pos = Pos { x: 0, y: 0 };

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos { x: i32, y: i32 }
impl Pos {
    fn unit(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}
impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}
impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Motion {
    dir: Pos,
    amount: i32,
}
impl Motion {
    fn new(dir: (i32, i32), amount: i32) -> Self {
        Self {
            dir: Pos { x: dir.0, y: dir.1 },
            amount,
        }
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
    let mut tail = ORIGIN;
    let mut head = ORIGIN;

    let mut tail_positions: HashSet<Pos> = HashSet::new();
    tail_positions.insert(tail);
    for m in motions {
        for _ in 0..m.amount {
            head += m.dir;
            let diff = head - tail;
            if diff.x.abs() >= 2 || diff.y.abs() >= 2 {
                tail += diff.unit();
                tail_positions.insert(tail);
            }
        }
    }

    tail_positions.len()
}

//=============== PART 2 ===============//
const KNOTS_N: usize = 10;

fn part2(motions: Vec<Motion>) -> usize {
    let mut knots = [Pos { x: 0, y: 0 }; KNOTS_N];

    let mut tail_positions = HashSet::new();
    tail_positions.insert(knots[KNOTS_N-1]);
    for m in motions {
        for _ in 0..m.amount {
            knots[0] += m.dir;
            for i in 1..knots.len() {
                let diff = knots[i-1] - knots[i];
                if diff.x.abs() >= 2 || diff.y.abs() >= 2 {
                    knots[i] += diff.unit();
                }
            }
            tail_positions.insert(knots[KNOTS_N-1]);
        }
    }

    tail_positions.len()
}

