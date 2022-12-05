//! Day 5 part 1 - Supply Stacks

use aoc22::line_reader::LineReader;

macro_rules! ascii_to_num {
    ($char_byte:expr) => {
        $char_byte - 0x30
    };
}

const INTERVAL: usize = 4;

fn parse_stacks(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut lines = lines.into_iter().rev();

    let last_line = lines.next().unwrap();
    let stacks_no = ascii_to_num!( last_line.as_bytes()[last_line.len()-2] ) as usize;

    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(lines.len()-1); stacks_no];

    for line in lines {
        let bytes = line.as_bytes();
        for i in 0..stacks_no {
            let char = bytes[1 + (i*INTERVAL)] as char;
            if char != ' ' {
                stacks[i].push(char);
            }
        }
    }

    stacks
}

fn main() {
    let mut lr = LineReader::new();

    // Parse stacks
    let mut lines = Vec::new();
    lr.read_next().unwrap();
    while !lr.line.is_empty() {
        lines.push(lr.line.clone());
        lr.read_next().unwrap();
    }

    let mut stacks = parse_stacks(lines);

    // Execute commands
    while lr.read_next().unwrap() > 0 {
        let nums = lr.as_numbers(10); //array of len 3 where [amount, src, dest]
        for _ in 0..nums[0] {
            if let Some(moved) = stacks[nums[1] as usize -1].pop() {
                stacks[nums[2] as usize -1].push(moved);
            }
        }
    }

    // Get answer
    let mut answer = String::with_capacity(stacks.len());
    for s in stacks {
        if let Some(c) = s.last() {
            answer.push(*c);
        }
    }

    println!("{}", answer)
}

