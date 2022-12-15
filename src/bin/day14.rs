//! Day 14: Regolith Reservoir

use std::collections::HashSet;

use aoc22::{solve_puzzle, position::Pos};

const SAND_SOURCE: Pos = Pos { x: 500, y: 0 };
const DOWN  : Pos = Pos { x:  0, y: 1 };
const DOWN_L: Pos = Pos { x: -1, y: 1 };
const DOWN_R: Pos = Pos { x:  1, y: 1 };

static mut LOWEST: i32 = 0;

fn main() {
    let mut tiles: HashSet<Pos> = HashSet::new();
    // Parse input
    let stdin = std::io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).unwrap() > 0 {
        let mut rock_edges = Vec::new();
        let s_coords = line.trim().split(" -> ");
        for s in s_coords {
            let mut n_coords = s.split(',')
                .filter_map(|str| str.parse::<i32>().ok());
            let pos = Pos::new(n_coords.next().unwrap(), n_coords.next().unwrap());
            rock_edges.push(pos);
        }

        for r in 0..rock_edges.len()-1 {
            let re1 = rock_edges[r];
            let re2 = rock_edges[r+1];
            if re1.x == re2.x {
                let upper = re1.y.min(re2.y);
                let lower = re1.y.max(re2.y);
                for y in upper..=lower {
                    tiles.insert(Pos::new(re1.x, y));
                }
                unsafe {
                    if lower > LOWEST {
                        LOWEST = lower;
                    }
                }
            }
            else {
                let left = re1.x.min(re2.x);
                let right = re1.x.max(re2.x);
                for x in left..=right {
                    tiles.insert(Pos::new(x, re1.y));
                }
            }
        }

        line.clear();
    }

    for y in 0..=168 {
        let mut line = String::with_capacity(503-494+1);
        for x in 491..=603 {
            line.push(
                if tiles.contains(&Pos::new(x, y)) { '#' }
                else if Pos::new(x,y) == SAND_SOURCE { '+' }
                else { '.' }
            );
        }
        println!("{line}");
    }
    println!();

    // Solve
    let answer = solve_puzzle!(tiles);
    println!("{answer}")
}

//=============== PART 1 ===============//
fn sand_fall(tiles: &mut HashSet<Pos>, mut sand: Pos) -> bool {
    let mut next_pos = sand + DOWN;
    while !tiles.contains(&next_pos) {
        sand = next_pos;
        next_pos += DOWN;
        if unsafe { sand.y >= LOWEST } {
            return false
        }
    }

    let old_pos = sand;
    if !tiles.contains(&(sand+DOWN_L)) {
        sand += DOWN_L;
    }
    else if !tiles.contains(&(sand+DOWN_R)) {
        sand += DOWN_R;
    }
    
    if sand == old_pos {
        tiles.insert(sand);
        true
    }
    else {
        sand_fall(tiles, sand)
    }
}

fn part1(mut tiles: HashSet<Pos>) -> i32 {
    let old_tiles = tiles.clone();

    let mut hour_glass = 0;
    while sand_fall(&mut tiles, SAND_SOURCE) {
        hour_glass += 1;
    }
    // Print new map
    for y in 0..=168 {
        let mut line = String::with_capacity(503-494+1);
        for x in 491..=603 {
            let pos = Pos::new(x, y);
            line.push(
                if old_tiles.contains(&pos) { '█' }
                else if tiles.contains(&pos) { 'O' }
                else if pos == SAND_SOURCE { '+' }
                else { ' ' }
            );
        }
        println!("{line}");
    }
    hour_glass
}

//=============== PART 2 ===============//
#[allow(unused_variables)]
fn part2(tiles: HashSet<Pos>) -> ! {
    todo!()
}

