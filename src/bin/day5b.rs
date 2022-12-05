//! Day 5 part 2 - Supply Stacks

use aoc22::line_reader::LineReader;

const INTERVAL: usize = 4;

fn parse_stacks(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut lines = lines.into_iter().rev();

    let last_line = lines.next().unwrap();
    let stacks_no = (last_line.as_bytes()[last_line.len()-2] as char)
                        .to_digit(10).unwrap() as usize;

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
        let split_i = stacks[nums[1]-1].len() - nums[0];
        let mut moved = stacks[nums[1]-1].split_off(split_i);
        stacks[nums[2]-1].append(&mut moved);
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

