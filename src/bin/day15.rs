//! Day 15: Beacon Exclusion Zone

use std::collections::{HashSet, HashMap};

use aoc22::{ solve_puzzle, position::Pos };

fn main() {
    if std::env::args().len() < 3 {
        aoc22::exit_msg!(1, "Usage: {} PART {{ROW_WANTED, COORDS_LIMIT}}", std::env::args().nth(0).unwrap());
    }

    let mut sb_pairs: Vec<(Pos,Pos)> = Vec::new();
    // Parse input
    let stdin = std::io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).unwrap() > 0 {
        let nums = line.split(&['=', ',', ':', '\n'])
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<i32>>();
        let pair = (
            Pos::new(nums[0], nums[1]),
            Pos::new(nums[2], nums[3])
        );
        sb_pairs.push(pair);

        line.clear();
    }

    // Solve
    let answer = solve_puzzle!(sb_pairs);
    println!("{answer}")
}

//=============== PART 1 ===============//
fn part1(signal_beacon_pairs: Vec<(Pos,Pos)>) -> i32 {
    let row_wanted = std::env::args().nth(2).expect("Row wanted not specified")
        .parse().expect("Row wanted should be an integer");

    let mut excluded = HashSet::new();

    for (signal, beacon) in signal_beacon_pairs {
        let dist = signal.manh_dist(&beacon);
        if !(signal.y - dist .. signal.y + dist).contains(&row_wanted) {
            continue
        }

        let col_range = dist - (signal.y - row_wanted).abs();
        for r_col in -col_range..=col_range {
            let g_col = signal.x + r_col;
            if (g_col, row_wanted) != (beacon.x, beacon.y) {
                excluded.insert(g_col);
            }
        }
    }

    excluded.len() as i32
}

//=============== PART 2 ===============//
// TODO
fn part2(signal_beacon_pairs: Vec<(Pos,Pos)>) -> i32 {
    let coords_lim: i32 = std::env::args().nth(2).expect("Coordenates upper limit not specified")
        .parse().expect("Coordenates limit should be an integer");
    let coords_lim_range = 0..=coords_lim;

    //                HashMap<row, HashSet<col>>
    let mut possible: HashMap<i32, HashSet<i32>> = coords_lim_range.clone()
        .map(|row| (
            row,
            coords_lim_range.clone().collect::<HashSet<i32>>()
        ))
        .collect();

    for (signal, beacon) in signal_beacon_pairs {
        let dist = signal.manh_dist(&beacon);
        for r_row in -dist..=dist {
            let g_row = signal.y + r_row;
            if !possible.contains_key(&g_row) {
                continue
            }

            let row = match possible.get_mut(&g_row) {
                Some(set) => set,
                None => continue
            };

            let col_range = dist - r_row.abs();
            for r_col in -col_range..=col_range {
                let g_col = signal.x + r_col;
                row.remove(&g_col);
            }

            if row.is_empty() {
                possible.remove(&g_row);
            }
        }
    }

    let beacon_pos = match possible.iter().next() {
        Some((row, set)) => match set.iter().next() {
            Some(col) => Pos::new(*col, *row),
            None => panic!("Set was empty at row {row}")
        }
        None => panic!("Map was empty")
    };

    beacon_pos.x * 4_000_000 + beacon_pos.y
}

