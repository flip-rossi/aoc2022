//! Day 6 parts 1 and 2 - Tuning Trouble

use std::process::exit;

use aoc22::circ_array::CircArray;

const SOP_LENGTH: usize = 4;
const SOM_LENGTH: usize = 14;

fn main() {
    // Read commandline argument for which part of the puzzle to solve
    let target = match std::env::args().nth(1) {
        Some(s) => {
            match usize::from_str_radix(&s, 10).expect("Part must be a number.") {
                1 => SOP_LENGTH,
                2 => SOM_LENGTH,
                _ => {
                    eprintln!("Part must be 1 or 2.");
                    exit(1)
                }
            }
        },
        None => {
            eprintln!("Please specify which part you want solved.");
            exit(1)
        }
    };

    // Read the line from stdin
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let line = buf.as_bytes();

    // Initialize circular array
    let mut circ = CircArray::new(target);
    let mut i = 0;
    while i < target {
        circ.add_last(line[i]);
        i+=1
    }

    let mut ans = None;

    while i < line.len() {
        circ.add_last(line[i]);
        i+=1;
        if !circ.has_duplicates() {
            ans = Some(i);
            break
        }
    }

    match ans {
        Some(n) => println!("{n}"),
        None => {
            eprintln!("Start-Of-Packet not found.");
            exit(1);
        },
    }
}

