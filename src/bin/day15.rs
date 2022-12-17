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
// TODO
fn part2(signal_beacon_pairs: Vec<(Pos,Pos)>) -> i64 {
    let coords_lim = match std::env::args().nth(2) {
        Some(arg) => arg.parse().expect("Coordenates limit should be an integer"),
        None => 4_000_000
    };

    // let possible_template = coords_lim_range.clone().collect::<HashSet<i32>>();

    for row in 0..=coords_lim {
        // eprintln!("ROW {row}");
        let mut blocked_ranges: Vec<(i32,i32)> = Vec::new();
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

            // eprintln!("{signal_range:?}");

            let mut not_merged = true;
            let mut i = 0;
            while i < blocked_ranges.len() {
                if signal_range.0-1 <= blocked_ranges[i].1 && blocked_ranges[i].0-1 <= signal_range.1 {
                    // merge_ranges(&mut blocked_ranges, signal_range);
                    let mut new_range = (
                        signal_range.0.min(blocked_ranges[i].0),
                        signal_range.1.max(blocked_ranges[i].1)
                    );
                    while let Some(next_r) = blocked_ranges.get(i+1) {
                        if next_r.0-1 <= new_range.1 {
                            new_range.0 = new_range.0.min(next_r.0);
                            new_range.1 = new_range.1.max(next_r.1);
                            blocked_ranges.remove(i+1);
                            // eprintln!("merged next");
                        }
                        else {
                            break
                        }
                    }
                    // let mut excluded = 0;
                    // // if i != 0 {
                    // //     if let Some(prev_r) = blocked_ranges.get(i-1) {
                    // //         if new_range.0-1 <= prev_r.1 {
                    // //             new_range.0 = new_range.0.min(prev_r.0);
                    // //             new_range.1 = new_range.1.max(prev_r.1);
                    // //             blocked_ranges.remove(i-1);
                    // //             excluded = 1;
                    // //             eprintln!("merged prev");
                    // //         }
                    // //     }
                    // // }
                    // eprintln!("new range {new_range:?}");
                    blocked_ranges[i] = new_range;
                    not_merged = false;
                    break
                }
                else if signal_range.1 < blocked_ranges[i].0-1 {
                    blocked_ranges.insert(i, signal_range);
                    break
                }
                i += 1;
            }
            if not_merged {
                blocked_ranges.push(signal_range);
            }
            // eprintln!("{blocked_ranges:?}");
        }
        // eprintln!("ROW {row}: {blocked_ranges:?}");
        if blocked_ranges.len() >= 2 { // || blocked_ranges[0].0 > 0 || blocked_ranges[blocked_ranges.len()-1].1 < coords_lim {
            eprintln!("{blocked_ranges:?} on x={}, y={row}", blocked_ranges[0].1+1);
            return (blocked_ranges[0].1+1) as i64 * 4_000_000 + row as i64
        }
        
    }

    panic!("empty space not found")
}

fn merge_ranges(ranges: &mut Vec<(i32,i32)>, mut signal_range: (i32,i32)) {
    let mut to_remove = Vec::with_capacity(ranges.len());
    for (i, r) in ranges.iter().enumerate() {
        if signal_range.0-1 <= r.1 && r.0-1 <= signal_range.1 {
            to_remove.push(i);
            signal_range = (
                signal_range.0.min(r.0),
                signal_range.1.max(r.1)
            );
        }
    }
    for &i in to_remove.iter().rev() {
        ranges.remove(i);
    }
    ranges.insert(to_remove[0], signal_range);
}

