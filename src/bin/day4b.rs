//! Day 4 part 2 - Camp Cleanup

use aoc22::line_reader::LineReader;

fn main() {
    let mut lr = LineReader::new();
    let mut answer = 0;

    while lr.read_next().unwrap() > 0 {
        let numbers = lr.line.split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if numbers[0] <= numbers[3] && numbers[2] <= numbers[1] {
            answer += 1;
        }
    }

    println!("Answer: {}", answer)
}

