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
fn part1(signal_beacon_pairs: Vec<(Pos,Pos)>) -> usize {
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

    excluded.len()
}

//=============== PART 2 ===============//
#[allow(unused_variables)]
fn part2(sb_pairs: Vec<(Pos,Pos)>) -> ! {
    todo!()
}

