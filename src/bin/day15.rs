//! Day 15: Beacon Exclusion Zone

use std::collections::HashSet;

use aoc22::{ solve_puzzle, position::Pos };

fn main() {
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
fn part1(signal_beacon_pairs: Vec<(Pos,Pos)>) -> i64 {
    let row_wanted = match std::env::args().nth(2) {
        Some(s) => s.parse().expect("Row wanted should be an integer"),
        None => 2_000_000
    };

    let mut excluded = HashSet::new();

    for (signal, beacon) in signal_beacon_pairs {
        let dist = signal.manh_dist(&beacon);
        if row_wanted < signal.y - dist || row_wanted > signal.y - dist {
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

    excluded.len() as i64
}

//=============== PART 2 ===============//
fn part2(signal_beacon_pairs: Vec<(Pos,Pos)>) -> i64 {
    let coords_lim = match std::env::args().nth(2) {
        Some(arg) => arg.parse().expect("Coordenates limit should be an integer"),
        None => 4_000_000
    };

    for row in 0..=coords_lim {
        let mut possible: Vec<(i32,i32)> = Vec::new();
        possible.push((0, coords_lim));
        for (signal, beacon) in &signal_beacon_pairs {
            let dist = signal.manh_dist(&beacon);

            let col_range = dist - (signal.y - row).abs();
            if col_range < 0 {
                continue
            }
            let signal_range = (
                (signal.x - col_range).clamp(0, coords_lim),
                (signal.x + col_range).clamp(0, coords_lim)
            );

            subtract_from_ranges(&mut possible, signal_range);

        }

        if let Some(r) = possible.first() {
            eprintln!("{possible:?} on x={}, y={row}", r.0);
            return r.0 as i64 * 4_000_000 + row as i64;
        }
    }

    panic!("empty space not found")
}

fn subtract_from_ranges(ranges: &mut Vec<(i32, i32)>, sub: (i32, i32)) {
    let mut new_ranges: Vec<(i32,i32)> = Vec::new();
    for i in 0..ranges.len() {
        let mut r = ranges[i];
        if !(sub.0 <= r.0 && sub.1 >= r.1) {
            if r.0-1 <= sub.1 && r.0-1 <= sub.1 {
                if r.0 >= sub.0 && r.1 > sub.1 {
                    //sub from left
                    r.0 = r.0.max(sub.1+1);
                }
                else if r.0 < sub.0 && r.1 <= sub.1 {
                    //sub from right
                    r.1 = r.1.min(sub.0-1);
                }
                else {
                    //split
                    let left_range = ( r.0, sub.0-1 );
                    new_ranges.push(left_range);
                    r.0 = sub.1+1;
                }
            }
            new_ranges.push(r);
        }
    }
    *ranges = new_ranges;
}

