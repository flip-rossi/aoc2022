//! Day 3 part 1 - Rucksack Reorganization

use aoc22::line_reader::LineReader;

fn get_letter_priority(ascii_letter: u8) -> usize {
    if ascii_letter <= 0x5A { // A-Z (0x41-0x5A)
        (ascii_letter as usize) - 0x40 + 26
    }
    else { // a-z (0x61-0x7A)
        (ascii_letter as usize) - 0x60
    }
}

fn main() {
    let mut lr = LineReader::new();
    let mut answer = 0;

    while lr.read_next().unwrap() > 0 {
        let items = lr.line.as_bytes(); // only chars in the A-Za-z range
        let mut counts = [0; 26*2];

        for i in 0..items.len()/2 {
            let p = get_letter_priority(items[i]);
            counts[p-1] += 1;
        }
        //check for repeated items in second compartment
        for i in items.len()/2..items.len() {
            let p = get_letter_priority(items[i]);
            if counts[p-1] > 0 {
                answer += p;
                counts[p-1] = 0; //only sum priority once
            }
        }
    }

    println!("Answer: {}", answer)
}

