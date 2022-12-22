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
fn part2(cubes: HashSet<(i32,i32,i32)>) -> i32 {
    let (mut min_pos, mut max_pos) = ((i32::MAX,i32::MAX,i32::MAX), (i32::MIN,i32::MIN,i32::MIN));
    for c in &cubes {
        min_pos = (
            min_pos.0.min(c.0),
            min_pos.1.min(c.1),
            min_pos.2.min(c.2)
        );
        max_pos = (
            max_pos.0.max(c.0),
            max_pos.1.max(c.1),
            max_pos.2.max(c.2)
        );
    }

    let mut adjacencies = std::collections::HashMap::new();

    for c in &cubes {
        for d in DIRECTIONS {
            let adjacent = (c.0+d.0, c.1+d.1, c.2+d.2);
            if !cubes.contains(&adjacent) {
                match adjacencies.get_mut(&adjacent) {
                    None => { adjacencies.insert(adjacent, 1); },
                    Some(count) => *count += 1,
                }
            }
        }
    }

    let mut internal = HashSet::with_capacity(adjacencies.len());
    for (&coord, _) in &adjacencies {
        let mut blocked_dirs = 0;
        for (i, d) in DIRECTIONS.iter().enumerate() {
            let mut check = coord;
            while check.0 >= min_pos.0 && check.1 >= min_pos.1 && check.2 >= min_pos.2
               && check.0 <= max_pos.0 && check.1 <= max_pos.1 && check.2 <= max_pos.2 {
                if cubes.contains(&check) {
                    blocked_dirs += 1;
                    break
                }
                check = (check.0+d.0, check.1+d.1, check.2+d.2);
            }
            if blocked_dirs <= i {
                break
            }
        }
        if blocked_dirs == 6 {
            internal.insert(coord);
        }
    }

    // Reassess some spaces in "internal" by verifying if they touch an external air cube
    let mut old_internal_len = 0;
    while old_internal_len != internal.len() {
        old_internal_len = internal.len();

        let mut now_external = HashSet::with_capacity(internal.len());
        for coord in &internal {
            for d in DIRECTIONS {
                let adjacent = (coord.0+d.0, coord.1+d.1, coord.2+d.2);
                if adjacencies.contains_key(&adjacent) && !internal.contains(&adjacent) {
                    now_external.insert(*coord);
                    break
                }
            }
        }

        for coord in now_external {
            internal.remove(&coord);
        }
    }

    for coord in internal {
        adjacencies.remove(&coord);
    }

    let mut surface_area = 0;
    for count in adjacencies.values() {
        surface_area += count;
    }

    surface_area
}

