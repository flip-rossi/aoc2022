//! Day 14: Regolith Reservoir

use std::collections::HashSet;

use aoc22::{solve_puzzle, position::Pos};

const SAND_SOURCE: Pos = Pos { x: 500, y: 0 };
const DOWN  : Pos = Pos { x:  0, y: 1 };
const DOWN_L: Pos = Pos { x: -1, y: 1 };
const DOWN_R: Pos = Pos { x:  1, y: 1 };

fn main() {
    let mut tiles: HashSet<Pos> = HashSet::new();
    let mut lowest = 0;
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
                if lower > lowest {
                    lowest = lower;
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

    let mut graphic = String::with_capacity(169*(603-491+1)+1);
    for y in 0..=168 {
        for x in 491..=603 {
            graphic.push(
                if tiles.contains(&Pos::new(x, y)) { '#' }
                else if Pos::new(x,y) == SAND_SOURCE { '+' }
                else { '.' }
            );
        }
        graphic.push('\n')
    }
    println!("{graphic}");

    // Solve
    let answer = solve_puzzle!(tiles, lowest);
    println!("{answer}")
}

fn drop_sand(tiles: &mut HashSet<Pos>, mut sand: Pos, floor: i32) -> Pos {
    loop {
        let mut next_pos = sand + DOWN;
        while !tiles.contains(&next_pos) {
            if next_pos.y >= floor {
                tiles.insert(sand);
                return sand
            }
            sand = next_pos;
            next_pos += DOWN;
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
            return sand
        }
    }
}

//=============== PART 1 ===============//
fn part1(mut tiles: HashSet<Pos>, lowest: i32) -> i32 {
    let old_tiles = tiles.clone();

    let mut hour_glass = 0;
    while drop_sand(&mut tiles, SAND_SOURCE, lowest+1).y < lowest {
        hour_glass += 1;
    }
    // Print new map
    for y in 0..=168 {
        let mut line = String::with_capacity(503-494+1);
        for x in 491..=603 {
            let pos = Pos::new(x, y);
            line.push(
                if old_tiles.contains(&pos) { '█' }
                else if tiles.contains(&pos) {
                    if pos.y == lowest { '~' }
                    else { 'O' }
                }
                else if pos == SAND_SOURCE { '+' }
                else { ' ' }
            );
        }
        println!("{line}");
    }
    hour_glass
}

//=============== PART 2 ===============//
fn part2(mut tiles: HashSet<Pos>, lowest: i32) -> i32 {
    let old_tiles = tiles.clone();

    let mut hour_glass = 1;
    while drop_sand(&mut tiles, SAND_SOURCE, lowest+2).y > 0 {
        hour_glass += 1;
    }
    // Print new map
    let mut graphic = String::with_capacity(171*(603-491+1)+1);
    for y in 0..=170 {
        for x in 491..=603 {
            let pos = Pos::new(x, y);
            graphic.push(
                if old_tiles.contains(&pos) || pos.y >= lowest+2 { '█' }
                else if tiles.contains(&pos) {
                    if pos == SAND_SOURCE { '@' }
                    else { 'o' }
                }
                else { ' ' }
            );
        }
        graphic.push('\n')
    }
    print!("{graphic}");

    hour_glass
}

