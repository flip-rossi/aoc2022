//! Day 18: Boiling Boulders

use std::collections::HashSet;

const DIRECTIONS: [(i32,i32,i32); 6] = [
    (-1, 0, 0), ( 1, 0, 0),
    ( 0,-1, 0), ( 0, 1, 0),
    ( 0, 0,-1), ( 0, 0, 1),
];

fn main() {
    let mut cubes: HashSet<(i32,i32,i32)> = HashSet::new();
    // Parse input
    for line in std::io::stdin().lines().map(|s| s.unwrap()) {
        let mut coords = line.trim().split(',').map(|num| num.parse().unwrap());
        let cube = ( coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap() );
        cubes.insert(cube);
    }

    // Solve
    let answer = aoc22::solve_puzzle!(cubes);
    println!("{answer}")
}

//=============== PART 1 ===============//
fn part1(cubes: HashSet<(i32,i32,i32)>) -> i32 {
    let mut exposed = 0;
    for c in &cubes {
        for d in DIRECTIONS {
            if !cubes.contains( &(c.0+d.0, c.1+d.1, c.2+d.2) ) {
                exposed += 1;
            }
        }
    }

    exposed
}

//=============== PART 2 ===============//
#[allow(unused_variables)]
fn part2(cubes: HashSet<(i32,i32,i32)>) -> ! {
    todo!()
}

